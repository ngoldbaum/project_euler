fn main() {
    println!("{}", sieve(2_000_000));
}

fn sieve(max: u64) -> u64 {
    let data: Vec<u64> = (0..max).collect();
    let mut isprime: Vec<bool> = vec![true; data.len()];
    isprime[0] = false;
    for (i, d) in data.iter().enumerate() {
        let offset: u64 = *d;
        if !isprime[i] || offset == 1 {
            continue;
        }
        for j in ((i as u64 + offset)..isprime.len() as u64).step_by(offset as usize) {
            isprime[j as usize] = false
        }
    }
    let mut primes = isprime
        .iter()
        .enumerate()
        .fold(vec![0u64; 0], |mut acc, (index, value)| {
            if *value {
                acc.push(index as u64)
            }
            acc
        });
    primes.remove(0);
    primes.iter().sum()
}
