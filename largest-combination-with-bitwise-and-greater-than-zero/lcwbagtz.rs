pub fn largest_combination(candidates: Vec<i32>) -> i32 {
    let cc = candidates.iter().map(|n| to_bits(*n)).collect::<Vec<_>>();
    (0..24)
        .into_iter()
        .map(|ind| cc.iter().filter(|c| c[ind] == '1').count())
        .max()
        .unwrap_or(0) as i32
}

fn to_bits(n: i32) -> Vec<char> {
    let mut result = vec!['0'; 24];
    let mut ind = 0;
    for c in format!("{:b}", n).chars().rev() {
        result[ind] = c;
        ind += 1
    }
    result.reverse();
    result
}

fn main() {
    //dbg!(to_bits(8));

    dbg!(largest_combination(vec![16, 17, 71, 62, 12, 24, 14]));
    dbg!(largest_combination(vec![8, 8]));
}
