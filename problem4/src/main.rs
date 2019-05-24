fn main() {
    let mut largest: u64 = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            let prod = i*j;
            if prod > largest && is_palindrome(prod) {
                largest = prod;
            }
        }
    }
    println!("{}", largest);        
}

fn is_palindrome(prod: u64) -> bool {
    let chars: Vec<char> = prod.to_string().chars().collect();
    let nchars = chars.len();
    let mut is_palindrome = true;
    for i in 0..nchars/2 {
        if chars[i] != chars[nchars - i - 1] {
            is_palindrome = false;
            break;
        }
    }
    is_palindrome
}
