fn main() {
    let numbers = (1..1001)
        .map(|n| englishify(n).replace(" ", "").replace("-", ""))
        .collect::<Vec<String>>();
    dbg!(&numbers);
    let nchars = numbers.iter().fold(0, |acc, x| acc + x.len());
    println!("{}", nchars);
}

fn englishify(i: usize) -> String {
    let ret = match i {
        1 => "one".to_owned(),
        2 => "two".to_owned(),
        3 => "three".to_owned(),
        4 => "four".to_owned(),
        5 => "five".to_owned(),
        6 => "six".to_owned(),
        7 => "seven".to_owned(),
        8 => "eight".to_owned(),
        9 => "nine".to_owned(),
        10 => "ten".to_owned(),
        11 => "eleven".to_owned(),
        12 => "twelve".to_owned(),
        13 => "thirteen".to_owned(),
        14 => "fourteen".to_owned(),
        15 => "fifteen".to_owned(),
        16 => "sixteen".to_owned(),
        17 => "seventeen".to_owned(),
        18 => "eighteen".to_owned(),
        19 => "nineteen".to_owned(),
        20 => "twenty".to_owned(),
        30 => "thirty".to_owned(),
        40 => "forty".to_owned(),
        50 => "fifty".to_owned(),
        60 => "sixty".to_owned(),
        70 => "seventy".to_owned(),
        80 => "eighty".to_owned(),
        90 => "ninety".to_owned(),
        1000 => "one thousand".to_owned(),
        _ => {
            let hundreds = i / 100;
            let tens = (i - hundreds * 100) / 10;
            let ones = i - hundreds * 100 - tens * 10;
            let mut ret: String = String::new();
            if hundreds > 0 {
                ret += &(englishify(hundreds) + " hundred");
            }
            if tens < 2 {
                let remainder = i - hundreds * 100;
                if remainder != 0 {
                    ret += &(" and ".to_owned() + &englishify(i - hundreds * 100));
                }
            } else {
                if hundreds > 0 {
                    ret += " and ";
                }
                ret += &englishify(tens * 10);
                if ones > 0 {
                    ret += &("-".to_owned() + &englishify(ones))
                }
            }
            ret
        }
    };
    ret
}
