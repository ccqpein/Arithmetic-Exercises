fn insertiong_sort(nl: &Vec<i32>, f: &Fn(&i32, &i32) -> bool) -> Vec<i32> {
    let mut head = vec![nl[0]];
    for i in &nl[1..] {
        if let Some(thisInd) = head.iter().position(|&r| f(&r, i)) {
            head.insert(thisInd, *i);
        } else {
            head.push(*i);
        }
        println!("{:?}", head);
    }
    return head;
}

fn large(a: &i32, b: &i32) -> bool {
    return a > b;
}

fn small(a: &i32, b: &i32) -> bool {
    return a < b;
}


fn main() {
    let test = vec![3, 7, 4, 9, 5, 2, 6, 1];
    insertiong_sort(&test, &large);
    insertiong_sort(&test, &small);
}
