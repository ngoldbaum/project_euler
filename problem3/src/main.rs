fn main() {
    let num: u64 = 600851475143;
    //let num: u64 = 13195;
    let mut largest: u64 = 0;

    let largest_possible = ((num as f64).sqrt() + 1.0) as u64;

    for i in (3..largest_possible).rev() {
        if i % 1_000_000_000 == 0 {
            println!("{}", i);
        }
        if i % 2 == 0 {
            continue;
        }
        if num % i == 0 {
            let mut is_prime = true;
            for j in 2..i {
                if i % j == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                largest = i;
                break;
            }
        }
    }
    println!("{}", largest);
}
