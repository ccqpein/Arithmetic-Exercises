pub fn concatenated_binary(n: i64) -> i64 {
    let mut result = 0;
    let mut space = 1;
    let mut count = 0;

    for i in 1..=n {
        //dbg!(i);
        if i >= space {
            space <<= 1;
            count += 1
        }
        //dbg!(space);
        //dbg!(count);
        //dbg!((result << count) + i);
        result = ((result << count) + i) % 1000000007;
    }

    result
}

fn main() {
    assert_eq!(concatenated_binary(1), 1);
    assert_eq!(concatenated_binary(3), 27);
    assert_eq!(concatenated_binary(12), 505379714);
}
