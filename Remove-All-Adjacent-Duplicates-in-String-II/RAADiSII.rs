pub fn remove_duplicates(s: String, k: i32) -> String {
    let mut stack: Vec<char> = vec![];
    let k = k as usize;
    let (mut last_count, mut last_count_vec, mut last) = (0, vec![], ' ');

    for c in s.chars() {
        println!(
            "last_count: {}, last_count_vec: {:?}, last: {}",
            last_count, last_count_vec, last
        );

        if c == last {
            last_count += 1;
            if last_count == k {
                stack.truncate(stack.len() - k + 1);
                last_count_vec.pop();
            }
        } else {
            last_count_vec.push(last_count + 1);
            last = c;
            last_count = 0;
        }
    }

    String::from_iter(stack)
}

fn main() {
    dbg!(remove_duplicates("deeedbbcccbdaa".into(), 3));
}
