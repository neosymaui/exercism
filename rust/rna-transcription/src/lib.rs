#[derive(Debug, PartialEq)]
pub struct DNA {
    data: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    data: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        check(dna, true)?;
        Ok(DNA {
            data: String::from(dna),
        })
    }

    pub fn into_rna(self) -> RNA {
        let mut res = String::new();

        for c in self.data.chars() {
            res.push(match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => '_',
            });
        }

        RNA {
            data: String::from(res),
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        check(rna, false)?;
        Ok(RNA {
            data: String::from(rna),
        })
    }
}

pub fn check(xna: &str, is_dna: bool) -> Result<(), usize> {
    let checked = if is_dna { "GCTA" } else { "CGAU" };
    if !xna.chars().all(|c| checked.contains(c)) {
        Err(xna.chars().position(|c| !checked.contains(c)).unwrap())
    } else {
        Ok(())
    }
}
