pub fn number_of_matches(mut n: i32) -> i32 {
    let mut s = 0;
    while n != 1 {
        s += n / 2;
        n = n - n / 2;
    }
    s
}

fn main() {
    assert_eq!(number_of_matches(7), 6);
    assert_eq!(number_of_matches(14), 13);
}
