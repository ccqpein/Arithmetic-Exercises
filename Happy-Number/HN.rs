fn split_num(s: &str) -> Vec<i32> {
    let a = s.chars();
    let result = a.map(|x| x.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    result
}

fn main() {
    let mut a = "123";
    let b = split_num(a);

    println!("{:?}", b);

    for c in a.chars() {
        println!("{:?}", c.to_string().parse::<i32>().unwrap());
    }
}
