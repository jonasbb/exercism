pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        PascalsTriangle::rows_helper(self.0)
    }

    fn rows_helper(rowcount: u32) -> Vec<Vec<u32>> {
        match rowcount {
            0 => Vec::new(),
            1 => vec![vec![1]],
            c => {
                let mut res = PascalsTriangle::rows_helper(c - 1);
                let mut tmp = Vec::new();
                // limit scope of borrow from res
                {
                    // calculate all rows by first always inserting a 1
                    // afterwards calculate the middle
                    // end with a second 1
                    let lastrow = res.last().unwrap(); // can never fail
                    tmp.push(1);
                    for (a, b) in lastrow.iter().zip(lastrow.iter().skip(1)) {
                        tmp.push(*a + *b);
                    }
                    tmp.push(1);
                }
                res.push(tmp);
                res
            }
        }
    }
}
