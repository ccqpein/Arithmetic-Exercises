fn checkPerfectNumber(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let mut result = 0;
    let x = num as f32;
    for i in 1..1 + x.sqrt() as u32 {
        if num % i == 0 {
            println!("{}", num / i);
            result += i + num / i;
        }
        //println!("{}", i);
    }
    if result - num == num {
        return true;
    }
    //println!("{}", x.sqrt() as u32);
    return false;
}

fn main() {
    println!("{}", checkPerfectNumber(10));
    println!("{}", checkPerfectNumber(28));
}
