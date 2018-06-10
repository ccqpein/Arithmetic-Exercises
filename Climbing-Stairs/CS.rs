use std::env;

fn climb_stair(total: i32) -> i32 {
    if total == 0 {
        return 1;
    }

    if total == 1 {
        return 1;
    }

    let re = climb_stair(total - 1) + climb_stair(total - 2);
    re
}

fn main() -> Result<(), String> {
    /*let testcase0 = 0;
    let testcase1 = 3;
    let testcase2 = 5;
    let testcase3 = 1;
    

    println!("{}", climb_stair(testcase0));
    println!("{}", climb_stair(testcase1));
    println!("{}", climb_stair(testcase2));
    println!("{}", climb_stair(testcase3));
     */

    let input = env::args().nth(1).unwrap();
    let input_num = input.parse().unwrap();
    println!("{}", climb_stair(input_num));

    Ok(())
}
