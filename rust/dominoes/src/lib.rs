// https://courses.cs.washington.edu/courses/cse143/11wi/lectures/02-16/18-recursive-backtracking-2.ppt
pub type Domino = (usize, usize);

pub fn chain(dominos: &[Domino]) -> Option<Vec<Domino>> {
    // test all possible chain start/end combinations
    for x in 1..6 + 1 {
        if let Some(x) = find_chain(Vec::from(dominos), x, x, dominos.len()) {
            return Some(x);
        }
    }
    // no chain found
    None
}

fn find_chain(mut dominos: Vec<Domino>,
              start: usize,
              end: usize,
              total_length: usize)
              -> Option<Vec<Domino>> {
    // a 0-length chain can only match, if start==end, then no more stones are necessary to
    // complete the circle
    if dominos.is_empty() {
        return if start == end {
            Some(Vec::with_capacity(total_length))
        } else {
            None
        };
    }

    // try to create a chain with all possible dominos
    for i in 0..dominos.len() {
        // choose explore stone
        let d = dominos.remove(i);
        // try normal and flipped
        if d.1 == end {
            if let Some(mut x) = find_chain(dominos.clone(), start, d.0, total_length) {
                x.push(d);
                return Some(x);
            }
        } else if flip_domino(&d).1 == end {
            if let Some(mut x) = find_chain(dominos.clone(),
                                            start,
                                            flip_domino(&d).0,
                                            total_length) {
                x.push(flip_domino(&d));
                return Some(x);
            }
        }

        // no chain found for this domino
        // insert into list and try next domino
        dominos.insert(i, d);
    }
    // no chains possible for all dominos
    None
}

/// Flips both sides of the Domino
fn flip_domino(domino: &Domino) -> Domino {
    (domino.1, domino.0)
}
