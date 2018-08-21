use std::str;

fn arr_u8_to_i32(input: &[u8]) -> i32 {
    if let Ok(r) = str::from_utf8(input) {
        if let Ok(rr) = r.parse::<i32>() {
            return rr;
        }
    }

    return 0;
}

fn butlast(input: &[u8]) -> Vec<u8> {
    let length = input.len();
    let mut re = vec![];

    for i in 0..length - 1 {
        re.push(input[i]);
    }
    re
}

fn complex_number_multiply<'a>(x: &'a str, y: &'a str) -> String {
    let x_temp: Vec<&str> = x.split('+').collect();
    let y_temp: Vec<&str> = y.split('+').collect();

    let inner_func = |a| arr_u8_to_i32(&butlast(a));

    let (ins, com) = match (x_temp[0].parse::<i32>(), y_temp[0].parse::<i32>()) {
        (Ok(x), Ok(y)) => (
            x * y - inner_func(x_temp[1].as_bytes()) * inner_func(y_temp[1].as_bytes()),
            x * inner_func(y_temp[1].as_bytes()) + y * inner_func(x_temp[1].as_bytes()),
        ),
        _ => return String::from("err"),
    };

    if ins != 0 {
        return format!(
            "{}{}{}{}",
            ins.to_string(),
            String::from("+"),
            com.to_string(),
            String::from("i"),
        );
    } else {
        return format!("0+{}{}", com.to_string(), String::from("i"),);
    }
}

fn main() {
    let test0 = complex_number_multiply("2+1i", "3+1i");
    //let a = str::from_utf8("123".as_bytes())
    //    .unwrap()
    //    .parse::<i32>()
    //    .unwrap();
    //println!("{:?}", a);

    //println!("{:?}", arr_u8_to_i32(&butlast("1234".as_bytes())));
    println!("{:?}", test0);

    let test1 = complex_number_multiply("1+1i", "1+1i");
    println!("{:?}", test1);

    let test2 = complex_number_multiply("1+-1i", "1+-1i");
    println!("{:?}", test2);
}
