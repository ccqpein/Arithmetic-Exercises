//use std::str::Split;
use std::rc::Rc;

struct NestedInteger {
    value: i32,
    next: Rc<NestedInteger>,
}

fn split_it(s: &str) -> Vec<&str> {
    let result = s.splitn(2, ",").collect();
    result
}

fn generator(s: &str) -> Rc<NestedInteger> {
    //let mut result = NestedInteger {};
    if s == "" {
        Rc::new()
    }

    let temp = split_it(s);

    if let Some(c) = temp[0].chars().next() {
        let mut temp_value: i32;
        if c != "[" {
            temp_value = temp[0].parse::<i32>().unwrap();
        } else {
            temp_value = temp[1..].parse::<i32>().unwrap();
        }
        //result.next = temp_value;
    }
    
    let result = NestedInteger {
        value : temp_value,
        next : generator(temp[1]),
    }
    //result.next = generator(temp[1]);

    return Rc::new(result);
}

fn main() {
    let test1 = "111";
    let test2 = "[111,[222,[333]]]";
    println!("{:?}", split_it(&test1));
    println!("{:?}", split_it(&test2));

    println!("{:?}", generator(test2));
}
