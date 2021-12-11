pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = vec![];
    for i in 1..limit {
        for j in factors.iter() {
            if j == &0u32 {
                break;
            }
            if i % j == 0 {
                multiples.push(i);
                break;
            }
        }
    }
    multiples.iter().sum()
}
