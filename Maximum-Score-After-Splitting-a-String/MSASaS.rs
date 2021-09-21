pub fn max_score(s: String) -> i32 {
    let (mut count, mut total_one) = (
        {
            if s.get(0..1).unwrap() == "0" {
                1_i32
            } else {
                0_i32
            }
        },
        {
            if s.get(s.len() - 1..s.len()).unwrap() == "1" {
                1_usize
            } else {
                0_usize
            }
        },
    );

    let mut max_count = count;

    for c in s.get(1..s.len() - 1).unwrap().chars() {
        match c {
            '0' => {
                count += 1;
                if count > max_count {
                    max_count = count
                }
            }
            '1' => {
                count -= 1;
                total_one += 1;
            }
            _ => unreachable!(),
        }
    }
    total_one as i32 + max_count
}

fn main() {
    assert_eq!(max_score("011101".to_string()), 5);
    assert_eq!(max_score("00111".to_string()), 5);
    assert_eq!(max_score("1111".to_string()), 3);
    assert_eq!(max_score("00".to_string()), 1);
}
