pub fn remove_duplicates(s: String, k: i32) -> String {
    let mut stack: Vec<char> = vec![];
    let k = k as usize;
    let mut cc = s.chars();
    let first = cc.next().unwrap();
    let (mut last_count, mut last_count_vec, mut last) = (1, vec![], first);
    stack.push(first);

    for c in cc {
        // println!(
        //     "last_count: {}, last_count_vec: {:?}, last: {}, stack: {:?}",
        //     last_count, last_count_vec, last, stack
        // );
        if c == last {
            last_count += 1;
            stack.push(c);

            if last_count == k {
                stack.truncate(stack.len() - k);
                //println!("stack here? {:?}", stack);
                last_count = last_count_vec.pop().unwrap_or(0);
                last = *stack.last().unwrap_or(&' ');
            }
        } else {
            last_count_vec.push(last_count);
            last = c;
            last_count = 1;
            stack.push(c);
        }
    }

    String::from_iter(stack)
}

fn main() {
    dbg!(remove_duplicates("deeedbbcccbdaa".into(), 3));
    dbg!(remove_duplicates("pbbcggttciiippooaais".into(), 2));
    dbg!(remove_duplicates("abcd".into(), 2));
}
