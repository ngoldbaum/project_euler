use num::bigint::{BigInt, ToBigInt};
use num::traits::{One, Zero};

fn main() {
    let mut num = 100.to_bigint().unwrap();
    let mut ret: BigInt = One::one();
    let zero: BigInt = Zero::zero();

    while num != zero {
        ret *= &num;
        num -= 1;
    }

    let digits = format!("{}", ret)
        .chars()
        .map(|c| c.to_digit(10).expect("not a digit!"))
        .collect::<Vec<u32>>();

    println!("{}", digits.iter().sum::<u32>());
}
