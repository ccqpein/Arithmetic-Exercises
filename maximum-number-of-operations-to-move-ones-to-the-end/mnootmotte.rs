pub fn max_operations(s: String) -> i32 {
    let mut result = 0;
    let mut count_1 = 0;
    let mut flag = false;
    for c in s.chars() {
        if c == '1' {
            if flag == true {
                flag = false;
                result += count_1
            }
            count_1 += 1;
            continue;
        }

        if c == '0' {
            flag = true;
        }
    }

    if flag {
        result += count_1
    }

    result
}

fn main() {
    dbg!(max_operations("1001101".to_string()));
    dbg!(max_operations("00111".to_string()));
    dbg!(max_operations("110".to_string()));
}
