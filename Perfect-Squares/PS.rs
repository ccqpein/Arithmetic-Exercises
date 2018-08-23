fn make_squares_list(n: i32) -> Vec<i32> {
    let mut re: Vec<i32> = vec![];
    for i in 0..n {
        let temp = i * i;
        if temp > n {
            break;
        } else {
            re.push(temp);
        }
    }

    re
}

fn main() {}
