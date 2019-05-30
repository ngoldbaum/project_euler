use std::error;
use std::fs::File;
use std::io::prelude::*;

use num::bigint::{BigInt, ToBigInt};

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn main() -> Result<()> {
    let filename = "data.txt";
    let window_size = 13;

    let contents = get_contents(filename)?.replace("\n", "");
    let mut big_prod: BigInt = 1u64.to_bigint().unwrap();

    for w in contents.chars().collect::<Vec<char>>().windows(window_size) {
        let prod = w
            .iter()
            .map(|c| c.to_digit(10).unwrap().to_bigint().unwrap())
            .fold(1u64.to_bigint().unwrap(), |acc, x| acc * x);
        if prod > big_prod {
            big_prod = prod;
        }
    }

    println!("{}", big_prod);

    Ok(())
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
