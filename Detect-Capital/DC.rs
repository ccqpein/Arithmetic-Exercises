fn detect_capital_use(word: &String) -> bool {
    let mut flag = -1;

    for (i, c) in word.chars().enumerate() {
        if i == 0 {
            if c.is_uppercase() {
                flag = 0;
            } else {
                flag = 2;
            }
            continue;
        }

        if i == 1 && flag != 2 {
            if c.is_uppercase() {
                flag = 1;
            } else {
                flag = 2;
            }
            continue;
        }

        if c.is_uppercase() && flag == 1 {
            continue;
        } else if !c.is_uppercase() && flag == 2 {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

fn main() {
    println!("{}", detect_capital_use(&String::from("USA")));
    println!("{}", detect_capital_use(&String::from("FlaG")));
    println!("{}", detect_capital_use(&String::from("ggg")));
}
