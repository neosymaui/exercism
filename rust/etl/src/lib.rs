use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut res: BTreeMap<char, i32> = BTreeMap::new();

    for (value, letters) in h {
        for letter in letters.iter() {
            res.insert(letter.to_lowercase().collect::<Vec<_>>()[0], *value);
        }
    }
    res
}
