fn main() {
    let maxnum = 100;
    let sumsquares: u64 = (1..maxnum+1).map(|x| x*x).sum();
    let sum: u64 = (1..maxnum+1).sum();
    let squaresum = sum*sum;
    println!("{}", squaresum - sumsquares);
}
