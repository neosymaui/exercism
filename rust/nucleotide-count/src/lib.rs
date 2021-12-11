use std::collections::HashMap;

const ACGT: &'static str = "ACGT";

pub fn check(dna: &str) -> Result<(), char> {
    for c in dna.chars() {
        if !ACGT.contains(c) {
            return Err(c);
        }
    }
    Ok(())
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !ACGT.contains(nucleotide) {
        Err(nucleotide)
    } else {
        check(dna)?;
        Ok(dna.matches(nucleotide).count())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    check(dna)?;

    let mut res: HashMap<char, usize> = HashMap::new();
    for c in ACGT.chars() {
        res.insert(c, dna.matches(c).count());
    }
    Ok(res)
}
