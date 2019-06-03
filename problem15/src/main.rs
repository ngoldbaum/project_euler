use num::bigint::{BigInt, ToBigInt};

fn main() {
    println!("{}", binomial(40, 20));
}

fn binomial(n: u64, k: u64) -> BigInt {
    let mut numerator: BigInt = 1.to_bigint().unwrap();
    let mut denominator: BigInt = 1.to_bigint().unwrap();

    for i in 2..n + 1 {
        numerator *= i.to_bigint().unwrap();
    }

    for i in 2..k + 1 {
        denominator *= i.to_bigint().unwrap();
    }

    for i in 2..((n - k) + 1) {
        denominator *= i.to_bigint().unwrap();
    }

    return numerator / denominator;
}
