pub struct Roman(u16);

impl Roman {
    pub fn to_string(&self) -> String {
        let mut res = String::new();
        // the vector defines which letters to use for which position
        for &(factor, oneer, fiveer, tener) in &[(1000, 'M', '-', '-'),
                                                 (100, 'C', 'D', 'M'),
                                                 (10, 'X', 'L', 'C'),
                                                 (1, 'I', 'V', 'X')] {
            match (self.0 / factor) % 10 {
                1 => res.push(oneer),
                2 => {
                    res.push(oneer);
                    res.push(oneer)
                }
                3 => {
                    res.push(oneer);
                    res.push(oneer);
                    res.push(oneer)
                }
                4 => {
                    res.push(oneer);
                    res.push(fiveer)
                }
                5 => res.push(fiveer),
                6 => {
                    res.push(fiveer);
                    res.push(oneer)
                }
                7 => {
                    res.push(fiveer);
                    res.push(oneer);
                    res.push(oneer)
                }
                8 => {
                    res.push(fiveer);
                    res.push(oneer);
                    res.push(oneer);
                    res.push(oneer)
                }
                9 => {
                    res.push(oneer);
                    res.push(tener)
                }
                0 | _ => {}
            }
        }
        res
    }
}

impl From<u16> for Roman {
    fn from(v: u16) -> Roman {
        Roman(v)
    }
}
