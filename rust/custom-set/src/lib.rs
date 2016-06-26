#[derive(Debug)]
pub struct CustomSet<T>(Vec<T>);

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn new(data: Vec<T>) -> Self {
        CustomSet(data)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn add(&mut self, value: T) {
        self.0.push(value)
    }

    pub fn contains(&self, value: &T) -> bool {
        self.0.contains(value)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.iter().all(|x| other.contains(x))
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0.iter().all(|x| !other.contains(x))
    }

    pub fn difference(&self, other: &Self) -> Self {
        CustomSet::new(self.0.iter().filter(|x| !other.contains(x)).cloned().collect())
    }

    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet::new(self.0.iter().filter(|x| other.contains(x)).cloned().collect())
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut tmp = self.0.clone();
        tmp.extend(other.0.clone());
        CustomSet::new(tmp)
    }
}

impl<T: PartialEq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}
