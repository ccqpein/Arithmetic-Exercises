use std::fmt::Debug;

fn cut_vec<T: Ord>(l: &mut Vec<T>) -> Vec<T> {
    let length = l.len() / 2;
    let result = l.split_off(length);
    result
}

fn binary_search<T: Ord + Clone + Debug>(l: &mut Vec<T>, k: T) -> bool {
    let length = l.len();
    if length == 0 {
        return false;
    }

    if length == 1 {
        match l.first() {
            Some(num) if *num == k => return true,
            _ => return false,
        }
    }

    let mut y = cut_vec(l);
    //println!("{:?}", &y);
    match y.clone().first() {
        Some(ref num) if **num > k => return binary_search(l, k),
        Some(_) => return binary_search(&mut y, k),
        _ => return false,
    }
}

fn main() {
    let v = vec![10, 14, 19, 26, 27, 31, 33, 35, 42, 44];
    println!("{:?}", binary_search(&mut v.clone(), 9));
    println!("{:?}", binary_search(&mut v.clone(), 45));
    println!("{:?}", binary_search(&mut v.clone(), 29));

    println!("{:?}", binary_search(&mut v.clone(), 10));
    println!("{:?}", binary_search(&mut v.clone(), 14));
    println!("{:?}", binary_search(&mut v.clone(), 31));
    println!("{:?}", binary_search(&mut v.clone(), 44));
}
