use std::fs::File;
use std::io::Read;

use num::bigint::BigInt;

fn main() -> Result<(), Box<std::error::Error>> {
    let filename = "data.txt";
    let contents = get_contents(filename)?;
    let numbers = contents
        .split("\n")
        .map(|num| BigInt::parse_bytes(num.as_bytes(), 10).expect("not a digit!"))
        .collect::<Vec<BigInt>>();
    let string_num = format!("{}", numbers.iter().sum::<BigInt>());
    println!("{}", &string_num[..10]);
    Ok(())
}

fn get_contents(filename: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
