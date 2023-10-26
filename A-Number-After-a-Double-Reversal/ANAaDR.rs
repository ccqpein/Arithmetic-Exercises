pub fn is_same_after_reversals(num: i32) -> bool {
    num == 0 || num % 10 != 0
}

fn main() {
    println!("{}", is_same_after_reversals(526));
    println!("{}", is_same_after_reversals(1800));
    println!("{}", is_same_after_reversals(0));
}
