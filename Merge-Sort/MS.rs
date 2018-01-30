fn merge(l1: Vec<i32>, l2: Vec<i32>) -> Vec<i32> {
    //println!("{:?} \n {:?} \n\n", l1, l2);
    if l1.len() == 0 {
        l2
    } else if l1[0] < l2[0] {
        let mut a = vec![l1[0]];
        let b = l1[1..].to_vec();
        a.extend(merge(b, l2));
        a
    } else {
        let mut a = vec![l2[0]];
        let b = l2[1..].to_vec();
        a.extend(merge(b, l1));
        a
    }
}

fn merege_out(l: Vec<i32>) -> Vec<i32> {
    let half = l.len() / 2;
    if l.len() >= 3 {
        return merge(
            merege_out(l[0..half].to_vec()),
            merege_out(l[half..].to_vec()),
        );
    } else {
        return merge(l[0..half].to_vec(), l[half..].to_vec());
    }
}

fn main() {
    let a = vec![
        1,
        2,
        3,
        2,
        4,
        3,
        2,
        4,
        2,
        1,
        3,
        4,
        3,
        2,
        1,
        2,
        4,
        5,
        6,
        7,
        7,
        5,
        4,
        5,
        4,
        2,
        2,
    ];
    println!("{:?}", merege_out(a));
}
