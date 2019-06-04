fn main() {
    let mut largest: u64 = 0;
    let mut start_largest: u64 = 0;

    for i in 2..1_000_000 {
        let length = n_collatz(i);
        if length > largest {
            largest = length;
            start_largest = i
        }
    }
    println!("{}", start_largest);
}

fn n_collatz(n: u64) -> u64 {
    let mut nloops = 0;
    let mut current = n;
    while current != 1 {
        nloops += 1;
        if current % 2 == 0 {
            current /= 2;
        } else {
            current = 3 * current + 1;
        }
    }
    nloops + 1
}
