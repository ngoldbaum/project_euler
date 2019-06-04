use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<std::error::Error>> {
    let filename = "data.txt";
    let contents = get_contents(filename)?;

    let mut rows = contents
        .split("\n")
        .map(|r| {
            r.split(" ")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let sum;

    loop {
        let row_to_destroy = match rows.pop() {
            Some(row) => row,
            None => {
                panic!();
            }
        };
        match rows.last_mut() {
            Some(last_row) => {
                for i in 0..last_row.len() {
                    if row_to_destroy[i] >= row_to_destroy[i + 1] {
                        last_row[i] += row_to_destroy[i];
                    } else {
                        last_row[i] += row_to_destroy[i + 1];
                    }
                }
            }
            None => {
                sum = row_to_destroy[0];
                break;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}

fn get_contents(filename: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
