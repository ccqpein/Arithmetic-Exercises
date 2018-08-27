fn filter_sequence(l: Vec<u32>) -> Vec<u32> {
    let length = l.len();
    let mut re: Vec<u32> = vec![];
    for i in (1..length).step_by(2) {
        re.push(l[i]);
    }
    re
}

fn last_remaining(n: u32) -> u32 {
    let a: Vec<u32> = (1..n + 1).collect();
    fn temp_func(ll: Vec<u32>) -> u32 {
        if ll.len() != 1 {
            let mut temp = filter_sequence(ll);
            temp.reverse();
            //println!("{:?}", temp);
            temp_func(temp)
        } else {
            *ll.first().unwrap()
        }
    }
    temp_func(a)
}

fn main() {
    //println!("{:?}", filter_sequence(vec![1, 2, 3, 4, 5, 6, 7]));
    println!("{:?}", last_remaining(10));
    println!("{:?}", last_remaining(9));
}
