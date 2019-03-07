pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut head = digits;
    let mut num_zero = 0;

    while let Some(&9) = head.last() {
        head = head.split_last().unwrap().1.to_vec();
        num_zero += 1;
    }

    //println!("{:?}", head);
    let mut last = 0;
    let mut head1vec = if let Some((last_temp, head1)) = head.split_last() {
        last = *last_temp;
        head1.to_vec()
    } else {
        vec![]
    };

    //let mut head1vec = head1.to_vec();
    head1vec.append(&mut vec![last + 1]);
    head1vec.append(&mut vec![0; num_zero]);

    head1vec
}

fn main() {
    println!("{:?}", plus_one(vec![1, 2, 3]));
    println!("{:?}", plus_one(vec![9]));
}
