use std::convert::TryFrom;

pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
    let mut primes = vec![2];

    let mut i = 3;
    while primes.len() < (usize::try_from(n).unwrap() + 1) {
        let mut push = true;
        for j in primes.clone().iter() {
            if i % j == 0 {
                println!("We skip {}", i);
                push = false;
                break
            }
        }
        if push == true {
            primes.push(i);
            println!("We add the prime number: {}", i);
        }
        i += 1;
    }
    primes[usize::try_from(n).unwrap()]
}
