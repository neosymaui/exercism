use std::collections::HashMap;

#[derive(Default)]
pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades
            .entry(grade)
            .or_insert_with(|| vec![])
            .push(String::from(student));
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut res = vec![];
        for g in self.grades.keys() {
            res.push(*g)
        }
        res.sort();
        res
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if self.grades.contains_key(&grade) {
            let mut res = self.grades.get(&grade).unwrap().clone();
            res.sort();
            Some(res)
        } else {
            None
        }
    }
}
