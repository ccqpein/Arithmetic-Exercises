use std::str::Split;

fn init_split(s: &str) -> Vec<&str> {
    let result = s.split(",").collect();
    result
}

fn main() {
    let test1 = "111,222";
    println!("{:?}", init_split(&test1));
}
