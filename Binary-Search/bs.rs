fn cut_vec<T: Ord>(l: &mut Vec<T>) -> Vec<T> {
    let length = l.len() / 2;
    let result = l.split_off(length);
    result
}

fn binary_search<T: Ord>(l: &mut Vec<T>, k: T) -> bool {
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
    //let head_y = y.first_mut();
    match y.first() {
        Some(ref num) if **num > k => return binary_search(l, k),
        Some(_) => return binary_search(&mut y, k),
        _ => return false,
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", cut_vec(&mut v));
    println!("{:?}", v);
    println!("{:?}", binary_search(&mut v, 3));
}
