pub struct Mapping(Vec<(&'static str, &'static str)>);

pub fn parse(data: Vec<(&'static str, &'static str)>) -> Mapping {
    Mapping(data)
}

impl Mapping {
    pub fn name_for(&self, short: &str) -> Result<&'static str, ()> {
        for &(x, y) in &self.0 {
            // assert that both iterators in the zip have the same length
            if short.chars().count() != x.chars().count() {
                continue;
            }

            // compare character for character
            let matches = short.chars().zip(x.chars()).all(|(a, b)| {
                match a {
                    // matches identity
                    'A' | 'C' | 'G' | 'T' => a == b,

                    'W' => b == 'A' || b == 'T',
                    'S' => b == 'C' || b == 'G',
                    'M' => b == 'A' || b == 'C',
                    'K' => b == 'G' || b == 'T',
                    'R' => b == 'A' || b == 'G',
                    'Y' => b == 'C' || b == 'T',

                    // matches everything except one
                    'B' => b != 'A',
                    'D' => b != 'C',
                    'H' => b != 'G',
                    'V' => b != 'T',

                    // matches all
                    'N' => true,

                    // unknown char matches nothing
                    _ => false,
                }
            });
            if matches {
                return Ok(y);
            }
        }
        // no match found
        Err(())
    }
}
