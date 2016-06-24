use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut res = BTreeMap::new();
    for (&points, values) in input {
        for value in values {
            res.insert(value.to_lowercase().to_string(), points);
        }
    }
    res
}
