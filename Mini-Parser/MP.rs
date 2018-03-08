//use std::str::Split;
use std::rc::Rc;

#[derive(Debug)]
struct NestedInteger {
    value: i32,
    next: Option<Rc<NestedInteger>>,
}

fn clean_tail(s: &str) -> &str {
    s.split(']').collect::<Vec<&str>>()[0]
}

fn split_it(s: &str) -> Vec<&str> {
    let result = s.splitn(2, ",").collect();
    result
}

fn generator(s: &str) -> Option<Rc<NestedInteger>> {
    //let mut result = NestedInteger {};
    if s == "" {
        return None;
    }

    let temp = split_it(s);
    let mut temp_value: i32 = 0;

    if let Some(c) = temp[0].chars().next() {
        if c != '[' {
            temp_value = temp[0].parse::<i32>().unwrap();
        } else {
            temp_value = temp[0][1..].parse::<i32>().unwrap();
        }
    }

    let result = Rc::new(NestedInteger {
        value: temp_value,
        next: if temp.len() > 1 {
            generator(temp[1])
        } else {
            None
        },
    });
    //result.next = generator(temp[1]);

    return Some(result);
}

fn main() {
    let test1 = "[111]]]";
    let test2 = "[111,[222,[333]]]";
    println!("{:?}", clean_tail(split_it(&test1)[0]));
    println!("{:?}", split_it(&test2));

    println!("{:?}", generator(clean_tail(test2)));
}
