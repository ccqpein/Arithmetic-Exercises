pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
    (0..1 << n).map(|i| start ^ i ^ i >> 1).collect()
}

fn main() {
    dbg!(circular_permutation(2, 3));
    dbg!(circular_permutation(3, 2));
}
