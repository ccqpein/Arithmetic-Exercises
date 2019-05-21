pub fn remove_duplicates(s: String) -> String {
    let bbs = s.as_bytes();
    let mut stack = vec![];
    let mut ind: i32 = -1;

    for i in bbs {
        if let Some(last) = stack.last() {
            if i != last {
                stack.push(*i);
                ind += 1
            } else {
                stack.remove(ind as usize);
                ind -= 1;
            }
        } else {
            stack.push(*i);
            ind += 1;
        }
    }

    String::from_utf8(stack).unwrap()
}

fn main() {
    println!("{:?}", remove_duplicates(String::from("abbaca")));
}
