pub fn calculate(s: String) -> i32 {
    //let mut bucket = vec![];
    let ss = clean(&s.chars().collect());
    helper2(&ss)
    //helper(&ss, None, ind)
}

fn helper2(cc: &Vec<String>) -> i32 {
    let mut bucket = vec![];
    let mut ind = 0;
    while ind < cc.len() {
        //dbg!(&ind);
        match cc.get(ind + 1) {
            Some(val) if *val == "+".to_string() || *val == "-".to_string() => {
                bucket.push(cc.get(ind).unwrap().to_string());
                bucket.push(cc.get(ind + 1).unwrap().to_string());
                ind += 2;
            }
            Some(val) if *val == "*".to_string() || *val == "/".to_string() => {
                bucket.push(group_mul_div_2(cc, &mut ind).to_string());
                ind += 1;
            }
            Some(_) => {
                bucket.push(cc.get(ind).unwrap().to_string());
                ind += 1
            }
            None => {
                bucket.push(cc.get(ind).unwrap().to_string());
                break;
            }
            _ => unreachable!(),
        }
    }
    dbg!(&bucket);

    let mut init = bucket[0].parse::<i32>().unwrap();
    let mut ind = 1;
    while ind < bucket.len() {
        match bucket[ind].as_str() {
            "+" => init += bucket[ind + 1].parse::<i32>().unwrap(),
            "-" => init -= bucket[ind + 1].parse::<i32>().unwrap(),
            _ => unreachable!(),
        }
        ind += 2
    }
    init
}

fn helper(cc: &Vec<String>, init: Option<i32>, mut ind: usize) -> i32 {
    dbg!(cc, init, ind);
    match cc.get(ind + 1) {
        Some(val) if *val == "+".to_string() => {
            init.unwrap_or(cc.get(ind).unwrap().parse::<i32>().unwrap()) + helper(cc, None, ind + 2)
        }
        Some(val) if *val == "-".to_string() => {
            init.unwrap_or(cc.get(ind).unwrap().parse::<i32>().unwrap()) - helper(cc, None, ind + 2)
        }
        Some(val) if *val == "*".to_string() => helper(cc, Some(group_mul_div(cc, &mut ind)), ind),
        Some(val) if *val == "/".to_string() => helper(cc, Some(group_mul_div(cc, &mut ind)), ind),
        None => cc.get(ind).unwrap().parse::<i32>().unwrap(),
        _ => unreachable!(),
    }
}

fn group_mul_div(cc: &Vec<String>, ind: &mut usize) -> i32 {
    dbg!(&ind);
    match cc.get(*ind + 1) {
        Some(val) if val == &"*".to_string() => {
            *ind += 2;
            cc.get(*ind - 2).unwrap().parse::<i32>().unwrap() * group_mul_div(cc, ind)
        }
        Some(val) if val == &"/".to_string() => {
            *ind += 2;
            cc.get(*ind - 2).unwrap().parse::<i32>().unwrap() / group_mul_div(cc, ind)
        }
        _ => cc.get(*ind).unwrap().parse::<i32>().unwrap(),
    }
}

fn group_mul_div_2(cc: &Vec<String>, ind: &mut usize) -> i32 {
    let mut init = cc[*ind].parse::<i32>().unwrap();
    *ind += 1;
    loop {
        match cc.get(*ind) {
            Some(val) if val == &"*".to_string() => {
                init *= cc.get(*ind + 1).unwrap().parse::<i32>().unwrap();
                *ind += 2
            }
            Some(val) if val == &"/".to_string() => {
                init /= cc.get(*ind + 1).unwrap().parse::<i32>().unwrap();
                *ind += 2
            }
            _ => {
                *ind -= 1;
                break;
            }
        }
    }
    init
}

fn clean(cc: &Vec<char>) -> Vec<String> {
    let mut result = vec![];
    let mut bucket = vec![];
    for i in 0..cc.len() {
        match cc[i] {
            '+' | '-' | '*' | '/' => {
                if bucket.len() != 0 {
                    result.push(merge_num(&bucket))
                }
                bucket.clear();
                result.push(cc[i].to_string());
            }
            ' ' => {
                if bucket.len() != 0 {
                    result.push(merge_num(&bucket))
                }
                bucket.clear();
            }
            _ => {
                bucket.push(cc[i]);
            }
        }
    }

    if bucket.len() != 0 {
        result.push(merge_num(&bucket))
    }
    result
}

fn merge_num(bucket: &Vec<char>) -> String {
    String::from_iter(bucket)
}

fn main() {
    dbg!(calculate("3+2*2".to_string()));
    dbg!(calculate("2*2".to_string()));
    dbg!(calculate(" 3/2 ".to_string()));
    dbg!(calculate(" 3+5 / 2 ".to_string()));

    assert_eq!(calculate("14/3*2".to_string()), 8);
}
