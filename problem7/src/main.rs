fn main() {
    println!("{}", sieve(10_001));
}

static MAX: usize = 1_000_000;

fn sieve(n: usize) -> usize {
    let data: Vec<usize> = (0..MAX).collect();
    let mut isprime: Vec<bool> = vec![true; data.len()];
    isprime[0] = false;
    for (i, d) in data.iter().enumerate() {
        let offset = *d;
        if !isprime[i] || offset == 1 {
            continue
        }
        for j in ((i+offset)..isprime.len()).step_by(offset) {
            isprime[j as usize] = false
        }
    }
    let primes = isprime.iter().enumerate().fold(vec![], |mut acc, (index, value)| {
        if *value {
            acc.push(index)
        }
        acc
    });
    primes[n]
}


