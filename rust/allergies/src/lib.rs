#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Allergen {
    Cats,
    Chocolate,
    Eggs,
    Peanuts,
    Pollen,
    Shellfish,
    Strawberries,
    Tomatoes,
}

impl Allergen {
    fn from(val: usize) -> Option<Self> {
        match val {
            1 => Some(Allergen::Eggs),
            2 => Some(Allergen::Peanuts),
            4 => Some(Allergen::Shellfish),
            8 => Some(Allergen::Strawberries),
            16 => Some(Allergen::Tomatoes),
            32 => Some(Allergen::Chocolate),
            64 => Some(Allergen::Pollen),
            128 => Some(Allergen::Cats),
            _ => None,
        }
    }
}

pub struct Allergies(Vec<Allergen>);

impl Allergies {
    pub fn new(mut allergies: usize) -> Allergies {
        let mut tmp = vec![];

        // iterate over all possible allergen values
        let mut x = 128;
        while x > 0 {
            // test for bit of allergen
            if x & allergies > 0 {
                // cannot fail
                tmp.push(Allergen::from(x).unwrap());
                allergies -= x;
            }
            x >>= 1;
        }
        tmp.reverse();
        Allergies(tmp)
    }

    /// Return list with all allergies
    pub fn allergies(&self) -> Vec<Allergen> {
        self.0.to_vec()
    }

    /// Check if it is in Allergies list
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.contains(allergen)
    }
}
