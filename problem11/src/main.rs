use ndarray::{Array, Ix2};
use std::fs::File;
use std::io::prelude::*;

struct Sequence((u64, u64, u64, u64));

impl Sequence {
    fn prod(&self) -> u64 {
        (self.0).0 * (self.0).1 * (self.0).2 * (self.0).3
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let filename = "data.txt";

    let contents = get_contents(filename)?;

    let array = get_array(&contents);

    let sh = array.shape();

    let nrows = sh[0];
    let ncols = sh[1];

    let mut largeprod = 0;

    for i in 0..nrows {
        for j in 0..ncols {
            // horizontal sequences
            check(horizontal_sequence(i, j, nrows, &array), &mut largeprod);
            check(vertical_sequence(i, j, nrows, &array), &mut largeprod);
            check(
                rising_diagonal_sequence(i, j, nrows, &array),
                &mut largeprod,
            );
            check(
                falling_diagonal_sequence(i, j, nrows, &array),
                &mut largeprod,
            );
        }
    }

    println!("{}", largeprod);

    Ok(())
}

fn get_contents(filename: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}

fn get_array(contents: &str) -> Array<u64, Ix2> {
    let rows = contents.split("\n").collect::<Vec<&str>>();

    let rows = rows
        .iter()
        .map(|r| {
            r.split(" ")
                .filter_map(|d| d.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let nrows = rows.len();
    let ncols = rows[0].len();

    let mut array = ndarray::Array::zeros((nrows, ncols));

    for i in 0..nrows {
        for j in 0..ncols {
            array[(i, j)] = rows[i][j];
        }
    }

    array
}

fn check(s: Option<Sequence>, largeprod: &mut u64) {
    match s {
        Some(x) => {
            let product = x.prod();
            if product > *largeprod {
                *largeprod = product
            }
        }
        None => {}
    }
}

fn horizontal_sequence(i: usize, j: usize, m: usize, arr: &Array<u64, Ix2>) -> Option<Sequence> {
    match (i, j) {
        (i, j) if i + 3 >= m || j + 3 >= m => None,
        _ => Some(Sequence((
            arr[(i, j)],
            arr[(i + 1, j)],
            arr[(i + 2, j)],
            arr[(i + 3, j)],
        ))),
    }
}

fn vertical_sequence(i: usize, j: usize, m: usize, arr: &Array<u64, Ix2>) -> Option<Sequence> {
    match (i, j) {
        (i, j) if i + 3 >= m || j + 3 >= m => None,
        _ => Some(Sequence((
            arr[(i, j)],
            arr[(i, j + 1)],
            arr[(i, j + 2)],
            arr[(i, j + 3)],
        ))),
    }
}

fn rising_diagonal_sequence(
    i: usize,
    j: usize,
    m: usize,
    arr: &Array<u64, Ix2>,
) -> Option<Sequence> {
    match (i, j) {
        (i, j) if i + 3 >= m || j + 3 >= m => None,
        _ => Some(Sequence((
            arr[(i, j)],
            arr[(i + 1, j + 1)],
            arr[(i + 2, j + 2)],
            arr[(i + 3, j + 3)],
        ))),
    }
}

fn falling_diagonal_sequence(
    i: usize,
    j: usize,
    m: usize,
    arr: &Array<u64, Ix2>,
) -> Option<Sequence> {
    match (i, j) {
        (i, j) if i + 3 >= m || j < 3 => None,
        _ => Some(Sequence((
            arr[(i, j)],
            arr[(i + 1, j - 1)],
            arr[(i + 2, j - 2)],
            arr[(i + 3, j - 3)],
        ))),
    }
}
