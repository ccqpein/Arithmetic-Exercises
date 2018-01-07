fn ad(i:u32) -> u32{
    let mut inner_int = i;
    while inner_int >= 10 {
        inner_int = (inner_int/10) + (inner_int%10);
    }
    return inner_int;
}

fn main() {
    println!("{}",ad(10));
    println!("{}",ad(38));
}
