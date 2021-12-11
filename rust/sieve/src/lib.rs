pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<(u64, bool)>;
    let mut res: Vec<u64> = vec![];

    if upper_bound > 1 {
        numbers = (0..upper_bound + 1).map(|x| (x, true)).collect();

        for i in 2..upper_bound + 1 {
            if numbers[i as usize].1 == true {
                res.push(i);

                let mut j = i;
                while i * j < upper_bound + 1 {
                    numbers[(i * j) as usize].1 = false;
                    j += 1;
                }
            }
        }
    }
    res
}
