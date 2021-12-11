pub struct Triangle {
    sides: Vec<u64>,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let temp = sides.to_vec();

        if temp.iter().any(|&v| v == 0) {
            None
        } else {
            let max = temp.iter().max().unwrap();
            let others: u64 = temp.iter().sum::<u64>() - max;

            if max > &others {
                None
            } else {
                let triangle = Triangle { sides: temp };
                Some(triangle)
            }
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[0] == self.sides[2]
    }
}
