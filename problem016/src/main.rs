use num::bigint::ToBigInt;

fn main() {
    let mut prod = 1.to_bigint().unwrap();
    let two = &prod + &prod;

    for _ in 1..1001 {
        prod *= &two;
    }

    let digits: Vec<u64> = format!("{}", prod)
        .chars()
        .map(|c| c.to_digit(10).expect("digit conversion failed") as u64)
        .collect();

    println!("{}", digits.iter().sum::<u64>());
}
