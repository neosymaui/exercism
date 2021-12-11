fn powered_digits_sum(n: u32) -> u32 {
    let p = n.to_string().len() as u32;
    let v: Vec<u32> = n.to_string().chars().map(|c| c.to_digit(10).unwrap().pow(p) as u32).collect();
    v.iter().sum()
}

pub fn is_armstrong_number(num: u32) -> bool {
    powered_digits_sum(num) == num
}
