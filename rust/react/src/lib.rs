#![feature(question_mark)]
use std::fmt::Debug;
use std::collections::HashMap;

struct Cell<'a, T: Copy + PartialEq + Debug> {
    value: T,
    next_callback_id: CallbackID,
    callbacks: HashMap<CallbackID, Box<FnMut(T) -> () + 'a>>,
    cell_type: CellType<'a, T>,
    /// a list of cells which depend on this cell and therefore need to be updated
    dependencies: Vec<CellID>,
    dirty: bool,
}

enum CellType<'a, T: Copy + PartialEq + Debug> {
    Input,
    Compute {
        inputs: Vec<CellID>,
        compute_func: Box<Fn(&[T]) -> T + 'a>,
    },
}

impl<'a, T: Copy + PartialEq + Debug> Cell<'a, T> {
    fn new_input(initial: T) -> Self {
        Cell {
            value: initial,
            next_callback_id: 0,
            callbacks: HashMap::new(),
            cell_type: CellType::Input,
            dependencies: Vec::new(),
            dirty: false,
        }
    }

    fn new_compute<F: Fn(&[T]) -> T + 'a>(initial: T,
                                          dependencies: &[CellID],
                                          compute_func: Box<F>)
                                          -> Self {
        Cell {
            value: initial,
            next_callback_id: 0,
            callbacks: HashMap::new(),
            cell_type: CellType::Compute {
                inputs: Vec::from(dependencies),
                compute_func: compute_func,
            },
            dependencies: Vec::new(),
            dirty: false,
        }
    }
}

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = usize;

// CellIDs are the offset within the cells array. Because cells can never be deleted
// the array is always dense and the check for valid CellID reduces to 'id < cells.len()'
pub struct Reactor<'a, T: Copy + PartialEq + Debug> {
    cells: Vec<Cell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
/// The callback structure requires that each cell triggers the callback at most once for each change,
/// even if the value would change multiple times, because multiple dependencies change.
///
/// Callbacks make a depth-first update strategy very easy to implement, however this can result in a
/// case where a compute cell (c3) is trying to update its value (triggered by c1) but c2 has not
/// yet update its own value. The update order would be c1, c3, c2.
///   i1
///  /  \
/// c1  c2
///  \  /
///   c3
///
/// Therefore, I use a mark then update strategy. First, all cells which depend on the changed input
/// get marked as dirty (via the callbacks). Afterwards the cells which are marked as dirty recompute
/// their values. The way the cells are created, they can only depend on previously created cells
/// so that the dependency structure forms an acyclic graph. Because the CellID is created
/// in ascending order updating the cells in ascending order will result in correct values.
/// The update process is triggered after the marking phase.
impl<'a, T: Copy + PartialEq + Debug> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor { cells: Vec::new() }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        let cell = Cell::new_input(initial);
        self.cells.push(cell);
        let new_cellid = self.cells.len() - 1;
        println!("Create input cell with ID: {}", new_cellid);
        new_cellid
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // Return an Err (and you can change the error type) if any dependency doesn't exist.
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(&mut self,
                                                 dependencies: &[CellID],
                                                 compute_func: F)
                                                 -> Result<CellID, &'static str> {
        // check that all dependencies are valid
        if !dependencies.iter().all(|x| self.is_valid_cellid(*x)) {
            return Err("Not all dependencies are valid.");
        }

        let initial = compute_func(dependencies.iter()
            .map(|id| self.value(*id).unwrap())
            .collect::<Vec<T>>()
            .as_ref());
        let compute_func = Box::new(compute_func);
        let cell = Cell::new_compute(initial, dependencies, compute_func);
        self.cells.push(cell);
        let new_cellid = self.cells.len() - 1;
        println!("Create compute cell with ID: {}", new_cellid);

        // notify other cells that this cell depends on them
        dependencies.iter().map(|id| self.cells[*id].dependencies.push(new_cellid)).count();

        Ok(new_cellid)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        // check for valid CellID
        if !self.is_valid_cellid(id) {
            None
        } else {
            println!("Value of cell ({}) is: {:?}", id, self.cells[id].value);
            Some(self.cells[id].value)
        }
    }

    // Sets the value of the specified input cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist, or the
    // specified cell is a compute cell, since compute cells cannot have their values directly set.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), &'static str> {
        // check for valid CellID
        if !self.is_valid_cellid(id) {
            Err("Invalid CellID")
        } else {
            // return value for input cells
            if let CellType::Input = self.cells[id].cell_type {
                if self.cells[id].value != new_value {
                    println!("Update value of cell ({}) to: {:?}", id, new_value);
                    self.cells[id].value = new_value;
                    self.mark_dirty(id);
                    self.cells[id].dirty = false;
                    self.update_dirty_cells();
                };
                Ok(())
            } else {
                Err("Cannot set_value on compute cells.")
            }
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> () + 'a>(&mut self,
                                                id: CellID,
                                                callback: F)
                                                -> Result<CallbackID, &'static str> {
        if !self.is_valid_cellid(id) {
            return Err("Invalid CellID");
        };
        let cell = &mut self.cells[id];
        let callbackid = cell.next_callback_id;
        cell.next_callback_id = callbackid + 1;
        cell.callbacks.insert(callbackid, Box::new(callback));
        Ok(callbackid)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Return an Err (and you can change the error type) if either the cell or callback
    // does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(&mut self,
                           id: CellID,
                           callbackid: CallbackID)
                           -> Result<(), &'static str> {
        if !self.is_valid_cellid(id) {
            return Err("Invalid CellID");
        };
        let cell = &mut self.cells[id];
        match cell.callbacks.remove(&callbackid) {
            Some(_) => Ok(()),
            None => Err("Invalid CallbackID"),
        }
    }

    fn is_valid_cellid(&self, id: CellID) -> bool {
        id < self.cells.len()
    }

    /// Recursively mark this cell and all other depended cells as dirty.
    fn mark_dirty(&mut self, id: CellID) {
        println!("Mark cell ({}) as dirty", id);
        self.cells[id].dirty = true;
        let deps: Vec<_> = self.cells[id].dependencies.iter().cloned().collect();
        deps.iter().map(|id| self.mark_dirty(*id)).count();
    }

    fn update_dirty_cells(&mut self) {
        println!("TEST1");
        (0..self.cells.len()).map(|id| self.update_cell(id)).count();
        println!("TEST2");
    }

    fn update_cell(&mut self, id: CellID) -> Result<(), &'static str> {
        println!("TEST");
        let inputs = {
            let ref cell = self.cells[id];
            // quick exit
            if !cell.dirty {
                return Ok(());
            }
            println!("Update dirty cell ({})", id);
            if let CellType::Compute { ref inputs, .. } = cell.cell_type {
                // clone such that no reference to self must will be hold afterwards
                inputs.clone()
            } else {
                return Err("Only compute cells can be updated.");
            }
        };
        let input_values = inputs.iter().map(|id| self.cells[*id].value).collect::<Vec<_>>();
        let ref mut cell = self.cells[id];
        if let CellType::Compute { ref compute_func, .. } = cell.cell_type {
            let newval = compute_func(input_values.as_ref());

            if cell.value != newval {
                // run callbacks if value changes
                cell.value = newval;
                for callback in cell.callbacks.values_mut() {
                    callback(newval)
                }
            }
        };
        Ok(())
    }
}
