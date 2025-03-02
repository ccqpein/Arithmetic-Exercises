pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let text1 = text1.into_bytes();
    let text2 = text2.into_bytes();

    let mut dp = vec![0; text1.len()];
    let mut am = false;
    let mut sc = 1;
    for i in 0..text2.len() {
        am = false;
        sc = 1;
        for j in 0..text1.len() {
            if sc <= dp[j] {
                am = false;
                sc += 1;
                continue;
            }

            if !am && text2[i] == text1[j] {
                dp[j] = sc;
                am = true
            }
        }
    }

    dp.into_iter().max().unwrap()
}

fn main() {}
