fn main() {
    let mut t1: u64 = 1;
    let mut t2: u64 = 2;
    let mut sum = 0;
    
    while t2 < 4_000_000 {
        if t2 % 2 == 0 {
            sum += t2
        }
        
        let temp = t1;
        t1 = t2;
        t2 = temp + t2;
    }

    println!("{}", sum);
}
