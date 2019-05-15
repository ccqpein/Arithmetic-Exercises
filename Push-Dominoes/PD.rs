#![feature(repeat_generic_slice)]
fn make_dot(dots: String, left: &str, right: &str) -> String {
    let length_dots = dots.len() as i32;
    let mut v = vec![];
    match (left, right) {
        ("L", "R") | ("", "R") | ("L", "") | ("", "") => {
            return dots;
        }
        ("L", "L") | ("", "L") => {
            v.append(&mut ['L'].repeat(length_dots as usize));
        }
        ("R", "R") | ("R", "") => {
            v.append(&mut ['R'].repeat(length_dots as usize));
        }
        ("R", "L") => {
            v.append(&mut ['R'].repeat((length_dots / 2) as usize));
            if length_dots % 2 == 1 {
                v.append(&mut vec!['.']);
            }
            v.append(&mut ['L'].repeat((length_dots / 2) as usize));
        }
        _ => {}
    }

    v.iter().collect::<String>()
}

fn push_dominoes(dominoes: String) -> String {
    let chars = dominoes.chars();
    let mut result: Vec<String> = vec![];
    let mut cache: Vec<char> = vec![];
    let mut last: char = ' ';

    for b in chars {
        if last != '.' && b == '.' {
            cache.push(b);
        } else if last == '.' && b != '.' {
            let cc = cache.iter().collect::<String>();
            if cc != "" {
                result.push(cc);
            }
            cache = vec![];
            result.push(b.to_string());
        } else if last == '.' && b == '.' {
            cache.push(b);
        } else {
            result.push(b.to_string());
        }

        last = b;
    }

    let cc = cache.iter().collect::<String>();
    if cc != "" {
        result.push(cc);
    }

    //println!("{:?}", result);

    let length = result.len();
    for ind in 0..length {
        if result[ind] != "L" && result[ind] != "R" {
            if ind == 0 {
                if length == 1 {
                    result[ind] = make_dot(result[ind].clone(), "", "");
                    continue;
                }
                result[ind] = make_dot(result[ind].clone(), "", &result[ind + 1]);
            } else if ind == length - 1 {
                result[ind] = make_dot(result[ind].clone(), &result[ind - 1], "");
            } else {
                result[ind] = make_dot(result[ind].clone(), &result[ind - 1], &result[ind + 1]);
            }
        }
    }

    //println!("{:?}", result);
    result.join("")
}

fn main() {
    assert_eq!(
        push_dominoes(String::from(".L.R...LR..L..")),
        String::from("LL.RR.LLRRLL..")
    );
    assert_eq!(push_dominoes(String::from("RR.L")), String::from("RR.L"));
    assert_eq!(push_dominoes(String::from(".")), String::from("."));
    assert_eq!(push_dominoes(String::from("LL")), String::from("LL"));

    //println!("{:?}", make_dot(String::from("..."), "L", "R"));
}
