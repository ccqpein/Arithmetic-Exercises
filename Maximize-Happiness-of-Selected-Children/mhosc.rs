pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
    happiness.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut result = 0;

    for (ind, h) in happiness.into_iter().enumerate().take(k as usize) {
        if h < ind as i32 {
            return result;
        }
        result += h as i64 - ind as i64
    }

    result
}

fn main() {}
