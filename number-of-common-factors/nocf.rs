pub fn common_factors(a: i32, b: i32) -> i32 {
    let mut count = 0;
    let mm = a.min(b);
    let mm = mm.min(1000);
    //dbg!(&mm);
    (1..=mm).for_each(|n| {
        if a % n == 0 && b % n == 0 {
            //dbg!(n);
            count += 1
        }
    });
    count
}

fn main() {
    assert_eq!(common_factors(12, 6), 4);
    assert_eq!(common_factors(25, 30), 2);
}
