pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
    let mut a = coordinate1.bytes();
    let a1 = a.next().unwrap() % 2 == 0;
    let a2 = a.next().unwrap() % 2 == 0;

    let mut b = coordinate2.bytes();
    let b1 = b.next().unwrap() % 2 == 0;
    let b2 = b.next().unwrap() % 2 == 0;

    (a1 == a2) == (b1 == b2)
}

fn main() {
    assert!(check_two_chessboards(
        String::from("a1"),
        String::from("c3")
    ));

    assert!(!check_two_chessboards(
        String::from("a1"),
        String::from("h3")
    ));
}
