fn main() {
    let mut count;
    for i in 1..1_000_000 {
        if i % 2 == 0 {
            count = nfactors(i / 2) * nfactors(i + 1);
        } else {
            count = nfactors(i) * nfactors((i + 1) / 2);
        }
        dbg!((i, i * (i + 1) / 2, count));
        if count > 500 {
            println!("{}, {}", count, i * (i + 1) / 2);
            break;
        }
    }
}

fn nfactors(n: u64) -> usize {
    if n == 1 {
        return 1;
    }

    let mut count = 2;

    let top = (n as f64).sqrt().floor() as u64;

    for i in 2..top + 1 {
        if n % i == 0 {
            count += 2;
        }
        if i * i == n {
            count -= 1;
        }
    }

    return count;
}
