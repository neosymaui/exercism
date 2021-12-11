pub fn raindrops(n: u32) -> String {
    // unimplemented!("what sound does Raindrop #{} make?", n)

    let mut res = String::new();

    if n % 3 == 0 {
        res.push_str("Pling")
    }
    if n % 5 == 0 {
        res.push_str("Plang")
    }
    if n % 7 == 0 {
        res.push_str("Plong")
    }
    if res.len() == 0 {
        res.push_str(&n.to_string())
    }
    res
}
