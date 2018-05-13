fn split_num(s: &str) -> Vec<i32> {
    let a = s.chars();
    let result = a.map(|x| x.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    result
}

fn is_happy(s: &str) -> bool {
    let num_vec: i32 = split_num(s).iter().sum();
    if num_vec < 1 {
        return false;
    }
    if num_vec == 1 {
        return true;
    }

    return is_happy(&num_vec.to_string());
}

fn main() {
    println!("{}", is_happy("19"));
    println!("{}", is_happy("1"));
}
