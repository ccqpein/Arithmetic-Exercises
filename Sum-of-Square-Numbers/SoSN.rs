fn judge_square_sum(input: i32) -> bool {
    if input == 0 {
        return true;
    }

    if input < 0 {
        return false;
    }

    let input_cnvrt = input as f32;

    for i in 0..input_cnvrt.sqrt() as i32 + 1 {
        let temp_i = i.pow(2);
        let mut temp_rest = (input - temp_i) as f32;
        temp_rest = temp_rest.sqrt();
        //println!("{}", temp_rest);
        if temp_rest == temp_rest.round() {
            return true;
        }
    }

    false
}
fn main() {
    println!("{}", judge_square_sum(0));
    println!("{}", judge_square_sum(5));
    println!("{}", judge_square_sum(2));

    println!("{}", judge_square_sum(3));
}
