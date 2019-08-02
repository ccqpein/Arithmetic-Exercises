use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn largest_number(nums: Vec<i32>) -> String {
    let mut cache: HashMap<u8, Vec<Vec<u8>>> = HashMap::new();
    for n in nums {
        let this = n.to_string();
        let bs = this.into_bytes();

        let this_value = cache.entry(bs[0]).or_insert(Vec::new());
        this_value.push(bs.clone());
    }

    //dbg!(&cache);

    let mut result: Vec<u8> = vec![];
    for i in 48..58 {
        if let Some(this_) = cache.get_mut(&(48 + 57 - i as u8)) {
            //this_.sort_by(compare);
            this_.sort_by(compare);
            this_.reverse();
            for v in this_ {
                result.append(v);
            }
        }
    }

    //dbg!(&result);
    if result[0] == 48 as u8 {
        String::from("0")
    } else {
        String::from_utf8(result).unwrap()
    }
}

fn compare_stupid(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
    let mut first = a.clone();
    first.append(&mut (b.clone()));
    let mut second = b.clone();
    second.append(&mut (a.clone()));

    let first_num = String::from_utf8(first).unwrap();
    let second_num = String::from_utf8(second).unwrap();

    if first_num > second_num {
        return Ordering::Greater;
    } else if first_num < second_num {
        return Ordering::Less;
    }

    return Ordering::Equal;
}

//give up
fn compare(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
    let mut ind = 0;
    let (lan_a, lan_b) = (a.len(), b.len());
    while ind != lan_a && ind != lan_b {
        if a[ind] > b[ind] {
            return Ordering::Greater;
        } else if b[ind] > a[ind] {
            return Ordering::Less;
        }

        ind += 1;
    }

    if ind < lan_a {
        while ind < lan_a {
            if a[ind] > a[ind - 1] {
                return Ordering::Greater;
            } else if a[ind] < a[ind - 1] {
                return Ordering::Less;
            }
            ind += 1;
        }
        return Ordering::Equal;
    } else if ind < lan_b {
        while ind < lan_b {
            if b[ind] > b[ind - 1] {
                return Ordering::Less;
            } else if b[ind] < b[ind - 1] {
                return Ordering::Greater;
            }
            ind += 1;
        }
        return Ordering::Equal;
    } else {
        return Ordering::Equal;
    }
}

fn compare_two<'x>(a: &'x Vec<u8>, b: &'x Vec<u8>) -> (bool, &'x Vec<u8>) {
    let mut ind = 0;
    let (lan_a, lan_b) = (a.len(), b.len());
    while ind != lan_a && ind != lan_b {
        if a[ind] > b[ind] {
            return (true, a);
        } else if b[ind] > a[ind] {
            return (false, b);
        }

        ind += 1;
    }

    if ind < lan_a {
        while ind < lan_a {
            if a[ind] > a[ind - 1] {
                return (true, a);
            } else if a[ind] < a[ind - 1] {
                return (false, b);
            }
            ind += 1;
        }
        return (true, a);
    } else if ind < lan_b {
        while ind < lan_b {
            if b[ind] > b[ind - 1] {
                return (false, b);
            } else if b[ind] < b[ind - 1] {
                return (true, a);
            }
            ind += 1;
        }
        return (false, b);
    } else {
        return (true, a);
    }
}

fn which_bit(a: String, b: String) {
    let a_ = a.into_bytes();
    let b_ = b.into_bytes();

    let len = a_.len();

    for i in 0..len {
        if a_[i] != b_[i] {
            println!("bit: {}, is: {}", i, a_[i] as char);
        }
    }
}

fn main() {
    dbg!(largest_number(vec![12, 23])); //=> 2312
    dbg!(largest_number(vec![10, 2])); //=> 210
    dbg!(largest_number(vec![3, 30, 34, 5, 9])); //=> 9534330
    dbg!(largest_number(vec![0, 0])); //=> 0

    let mut a = vec![824, 938, 1399, 5607, 6973, 5703, 9609, 4398, 8247];
    dbg!(largest_number(a));

    a = vec![2, 2281];
    dbg!(largest_number(a));
}
