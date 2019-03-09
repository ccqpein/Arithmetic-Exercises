pub fn convert_to_title_old(n: i32) -> String {
    let mut temp_l = vec![];
    let mut inner_n = n;

    inner_n = n;
    let mut rest = 0;

    println!("{},{}", inner_n, rest);
    while inner_n > 26 {
        rest = inner_n % 26;
        temp_l.push((rest + 64) as u8);

        inner_n = inner_n / 26
    }

    temp_l.push((inner_n + 64) as u8);

    let mut result = vec![];
    for c in temp_l.iter().rev() {
        result.push(*c);
    }
    String::from_utf8(result).unwrap()
}

pub fn convert_to_title(n: i32) -> String {
    let mut temp_l = vec![];
    let mut inner_n = n;

    while inner_n > 26 {
        if inner_n % 26 == 0 {
            temp_l.push((26 + 64) as u8);
            inner_n = inner_n / 26 - 1;
            continue;
        }

        temp_l.push((inner_n % 26 + 64) as u8);
        inner_n = inner_n / 26;
    }

    // print out
    temp_l.push((inner_n + 64) as u8);

    let mut result = vec![];
    for c in temp_l.iter().rev() {
        result.push(*c);
    }
    String::from_utf8(result).unwrap()
}

fn main() {
    //println!("{}", 5 / 2);
    println!("{}", convert_to_title(28)); //AB
    println!("{}", convert_to_title(701)); //ZY
    println!("{}", convert_to_title(26)); //Z
    println!("{}", convert_to_title(52)); //AZ
}
