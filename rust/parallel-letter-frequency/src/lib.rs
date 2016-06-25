use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::sync_channel;
use std::thread;

#[derive(PartialEq,Eq)]
pub struct ParallelFrequency(HashMap<char, u32>);

pub fn frequency(data: &[&str], worker: usize) -> HashMap<char, u32> {
    // need to create a copy of the data so that all threads have access
    // already prepare for case insensitivity
    let data = Arc::new(data.into_iter().map(|x| x.to_lowercase()).collect::<Vec<String>>());

    let mut children = Vec::with_capacity(worker);
    for workernumber in 0..worker {
        // setup communication channels
        let (child_tx, main_rx) = sync_channel(0);
        let data = data.clone();

        let child = thread::spawn(move || {
            // setup counter
            let mut res = HashMap::new();

            for i in 0..data.len() {
                if i % worker == workernumber {
                    // only alphabetical character
                    for c in data[i].chars().filter(|c| c.is_alphabetic()) {
                        // increase count
                        let mut count = res.entry(c).or_insert(0);
                        *count += 1;
                    }
                }
            }

            // send results back to main channel
            // cannot fail, because main thread still exists with other endpoint
            child_tx.send(res).unwrap();
        });

        children.push((child, main_rx));
    }

    let mut res = HashMap::new();
    // synchronize
    for child in children {
        let tmp = child.1.recv().unwrap();
        for (c, ccount) in tmp {
            let mut count = res.entry(c).or_insert(0);
            *count += ccount;
        }
        child.0.join().unwrap();
    }
    res
}
