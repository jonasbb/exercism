use std::collections::HashMap;

pub struct School(HashMap<u8, Vec<String>>);

impl School {
    pub fn new() -> School {
        School(HashMap::new())
    }

    /// Vector of all distinct grades in ascending order
    pub fn grades(&self) -> Vec<u8> {
        // map to convert & to value
        let mut ret: Vec<u8> = self.0.keys().map(|x| *x).collect();
        // must be in order
        ret.sort();
        ret
    }

    /// Vector of all names or None if no names are found
    pub fn grade(&self, grade: u8) -> Option<&Vec<String>> {
        self.0.get(&grade)
    }

    /// Add name and grade combination to School
    pub fn add(&mut self, grade: u8, name: &str) {
        let val = self.0.entry(grade).or_insert(Vec::new());
        val.push(name.to_string());
        // store values in order, insertion sort would be more efficient
        val.sort()
    }
}
