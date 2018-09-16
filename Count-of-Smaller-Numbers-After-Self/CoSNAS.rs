fn count_smaller(a: &Vec<i32>) -> Vec<i32> {
    let mut inner_a = a.clone();
    let mut result: Vec<i32> = vec![];

    while (!inner_a.is_empty()) {
        let temp = inner_a[0];
        result.push(inner_a[1..].iter().filter(|x| x < &&temp).count() as i32);
        inner_a = inner_a[1..].to_vec();
    }
    result
}

fn main() {
    let testcase0 = vec![5, 2, 6, 1];
    println!("{:?}", count_smaller(&testcase0));
}
