fn main() {
    let mut test_int: u64 = 20;
    let mut all_divisible = false;
    while !all_divisible {
        test_int += 1;
        all_divisible = true;
        for i in 1..21 {
            if test_int % i != 0 {
                all_divisible = false
            }      
        }
        if all_divisible {
            break;
        }
    }
    println!("{}", test_int)
}
