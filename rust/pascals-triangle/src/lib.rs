pub struct PascalsTriangle {
    pub rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut p = PascalsTriangle {
            rows: Vec::with_capacity(row_count as usize),
        };

        for r in 0..row_count {
            p.build_row(r);
        }
        p
    }

    pub fn rows(self) -> Vec<Vec<u32>> {
        self.rows
    }

    fn build_row(&mut self, row_num: u32) {
        let mut value = 1_u32;
        let mut new_row: Vec<u32> = vec![value];

        if row_num == 0 {
            self.rows.push(new_row);
        } else {
            if let Some(previous_row) = self.rows.get((row_num - 1) as usize) {
                for i in 1..row_num + 1 {
                    if let Some(a) = previous_row.get((i - 1) as usize) {
                        value = *a;
                    };
                    if let Some(b) = previous_row.get((i) as usize) {
                        value += *b;
                    };
                    new_row.push(value);
                }
            };
            self.rows.push(new_row);
        }
    }
}
