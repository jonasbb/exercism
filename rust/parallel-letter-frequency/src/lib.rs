use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;

pub type Counter = HashMap<char, usize>;

pub fn frequency(data: &[&str], worker: usize) -> Counter {
    // need to create a copy of the data so that all threads have access
    // already prepare for case insensitivity
    let data = Arc::new(data.into_iter().map(|x| x.to_lowercase()).collect::<Vec<String>>());

    // setup communication channels
    let (child_tx, main_rx) = channel();
    let mut children = Vec::with_capacity(worker);
    for workernumber in 0..worker {
        let data = data.clone();
        let child_tx = child_tx.clone();

        let child = thread::spawn(move || {
            // setup counter
            let mut res = HashMap::new();
            for i in 0..data.len() {
                if i % worker == workernumber {
                    freq(&mut res, &data[i]);
                }
            }

            // send results back to main channel
            // cannot fail, because main thread still exists with other endpoint
            child_tx.send(res).unwrap();
        });

        children.push(child);
    }

    let mut res = HashMap::new();
    // get data
    for _ in 0..worker {
        let tmp = main_rx.recv().unwrap();
        merge(&mut res, &tmp);
    }
    // synchronize threads
    for child in children {
        child.join().unwrap();
    }
    res
}

fn freq(counter: &mut Counter, data: &str) {
    for c in data.chars().filter(|c| c.is_alphabetic()) {
        let mut count = counter.entry(c).or_insert(0);
        *count += 1;
    }
}

fn merge(c1: &mut Counter, c2: &Counter) {
    for (&key, &value) in c2 {
        let mut count = c1.entry(key).or_insert(0);
        *count += value;
    }
}
