fn insertiong_sort(nl: Vec<i32>, f: &Fn(&i32, &i32) -> bool) -> Vec<i32> {
    let mut head = vec![nl[0]];
    for i in &nl[1..] {
        let thisInd = head.iter().position(|&r| f(r,i)).unwrap();
        if thisInd 
    }
}

fn large(a: &i32, b: &i32) -> bool {
    return a > b;
}

fn small(a: &i32, b: &i32) -> bool {
    return a < b;
}
