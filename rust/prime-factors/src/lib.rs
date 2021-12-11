pub fn factors(n: u64) -> Vec<u64> {
    let mut m = n;
    let mut primes = vec![];
    let mut i = 2_u64;

    while i * i <= m {
        if m % i == 0 {
            m /= i;
            primes.push(i);
        } else {
            i += 1;
        }
    }
    if m > 1 {
        primes.push(m);
    }
    primes
}
