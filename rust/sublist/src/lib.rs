#[derive(Debug,PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match (sublist_of(a, b), sublist_of(b, a)) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}

fn sublist_of<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() > b.len() {
        // a larger list can nerver be a sublist
        return false;
    } else {
        // for all possible ways to missalign the list, test for sublist
        for shift in 0..(b.len() - a.len()) + 1 {
            if b.iter().skip(shift).zip(a.iter()).all(|(ref x, ref y)| x == y) {
                // all elements matched
                return true;
            }
        }
        // no match found
        return false;
    }
}
