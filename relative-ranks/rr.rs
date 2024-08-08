pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut x: [Option<usize>; 1000001] = [None; 1000001];
    let len = score.len();

    for (ind, s) in score.into_iter().enumerate() {
        x[s as usize] = Some(ind);
    }

    let mut res = vec![String::new(); len];
    let mut a = 1;
    for xx in x.into_iter().rev() {
        if let Some(ind) = xx {
            match a {
                1 => res[ind] = "Gold Medal".to_string(),
                2 => res[ind] = "Silver Medal".to_string(),
                3 => res[ind] = "Bronze Medal".to_string(),
                _ => res[ind] = format!("{}", a),
            }

            a += 1;
        }
    }
    res
}

fn main() {
    dbg!(find_relative_ranks(vec![5, 4, 3, 2, 1]));
}
