fn complex_number_multiply<'a>(x: &'a str, y: &'a str) -> String {
    let mut x_temp: Vec<&str> = x.split('+').collect();
    let mut y_temp: Vec<&str> = y.split('+').collect();

    println!("{:?}", x_temp);
    println!("{:?}", y_temp);
    println!("{:?}", y_temp[1].as_bytes()[1]);

    let ins = x_temp[0].parse::<u32>().unwrap() * y_temp[0].parse::<u32>().unwrap();
    //let com = y_temp
    println!("{:?}", ins);

    return String::from("aa");
}

fn main() {
    complex_number_multiply("2+1i", "3+1i");
}
