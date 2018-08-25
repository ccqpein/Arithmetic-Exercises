fn filter_sequence(l: Vec<u32>) -> Vec<u32> {
    let length = l.len();
    let mut re: Vec<u32> = vec![];
    for i in (1..length - 1).step_by(2) {
        re.push(l[i]);
    }
    re
}

fn main() {
    println!("{:?}", filter_sequence(vec![1, 2, 3, 4, 5, 6, 7]));
}
