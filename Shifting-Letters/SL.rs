pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
    let mut sum = 0;
    let mut s_b = s.bytes().rev();
    let mut result: Vec<u8> = vec![];

    for i in shifts.iter().rev() {
        sum += i;
        sum %= 26;
        let mut this_one = s_b.next().unwrap() as i32 + sum;
        if this_one > 122 {
            this_one -= 26;
        }

        result.push(this_one as u8);
    }

    result.reverse();
    String::from_utf8(result).unwrap()
    // match String::from_utf8(result) {
    //     Ok(s) => s,
    //     Err(e) => {
    //         println!("{:?}", e);
    //         String::new()
    //     }
    // }
}

fn main() {
    //println!("{:?}", 97 as u8);
    dbg!(shifting_letters(String::from("abc"), vec![3, 5, 9])); //=> rpl
    dbg!(shifting_letters(String::from("bad"), vec![10, 20, 30])); //=> jyh
    dbg!(shifting_letters(
        String::from("mkgfzkkuxownxvfvxasy"),
        vec![
            505870226, 437526072, 266740649, 224336793, 532917782, 311122363, 567754492, 595798950,
            81520022, 684110326, 137742843, 275267355, 856903962, 148291585, 919054234, 467541837,
            622939912, 116899933, 983296461, 536563513
        ]
    )); //=>
}
