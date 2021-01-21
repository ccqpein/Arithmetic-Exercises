pub fn decode(encoded: Vec<i32>, mut first: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(encoded.len() + 1);
    result.push(first);
    for i in encoded {
        first = i ^ first;
        result.push(first);
    }
    result
}

fn main() {
    assert_eq!(decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
    assert_eq!(decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
}
