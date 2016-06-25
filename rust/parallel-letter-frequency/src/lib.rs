use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub type Counter = HashMap<char, usize>;

pub fn frequency(data: &[&'static str], worker: usize) -> Counter {
    // need to create a copy of the data so that all threads have access
    let data = Arc::new(Vec::from(data));

    let mut children = Vec::with_capacity(worker);
    for workernumber in 0..worker {
        let data = data.clone();

        let child = thread::spawn(move || {
            // setup counter
            let mut res = HashMap::new();
            for i in 0..data.len() {
                if i % worker == workernumber {
                    freq(&mut res, &data[i]);
                }
            }
            res
        });

        children.push(child);
    }

    let mut res = HashMap::new();
    // synchronize threads
    for child in children {
        let tmp = child.join().ok().expect("Could not join a thread!");
        merge(&mut res, &tmp);
    }
    res
}

fn freq(counter: &mut Counter, data: &str) {
    // to_lowercase() returns an iterator, so we need to access it
    for c in data.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_lowercase().next().unwrap()) {
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
