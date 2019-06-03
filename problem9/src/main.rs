fn main() {
    if let Some(sums) = allsums(1000) {
        for s in sums {
            if s.0.pow(2) + s.1.pow(2) == s.2.pow(2) {
                dbg!(s.0 * s.1 * s.2);
            }
        }
    }
}

fn allsums(sumtarget: u64) -> Option<Vec<(u64, u64, u64)>> {
    let mut ret: Vec<(u64, u64, u64)> = Vec::new();

    for i in (1..sumtarget).rev() {
        for j in (2..sumtarget - i).rev() {
            ret.push((i, j, sumtarget - i - j));
        }
    }

    if ret.len() == 0 {
        return None;
    }
    Some(ret)
}
