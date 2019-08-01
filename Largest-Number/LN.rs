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
    //let a = String::from("123").into_bytes();
    //let b = String::from("123").into_bytes();
    //dbg!(compare_two(&a, &b));

    dbg!(largest_number(vec![12, 23])); //=> 2312
    dbg!(largest_number(vec![10, 2])); //=> 210
    dbg!(largest_number(vec![3, 30, 34, 5, 9])); //=> 9534330
    dbg!(largest_number(vec![0, 0])); //=> 0

    dbg!(largest_number(vec![
        4704, 6306, 9385, 7536, 3462, 4798, 5422, 5529, 8070, 6241, 9094, 7846, 663, 6221, 216,
        6758, 8353, 3650, 3836, 8183, 3516, 5909, 6744, 1548, 5712, 2281, 3664, 7100, 6698, 7321,
        4980, 8937, 3163, 5784, 3298, 9890, 1090, 7605, 1380, 1147, 1495, 3699, 9448, 5208, 9456,
        3846, 3567, 6856, 2000, 3575, 7205, 2697, 5972, 7471, 1763, 1143, 1417, 6038, 2313, 6554,
        9026, 8107, 9827, 7982, 9685, 3905, 8939, 1048, 282, 7423, 6327, 2970, 4453, 5460, 3399,
        9533, 914, 3932, 192, 3084, 6806, 273, 4283, 2060, 5682, 2, 2362, 4812, 7032, 810, 2465,
        6511, 213, 2362, 3021, 2745, 3636, 6265, 1518, 8398
    ])); // => 98909827968595339456944893859149094902689398937839883538183810810780707982784676057536747174237321720571007032685668066758674466986636554651163276306626562416221603859725909578457125682552954605422520849804812479847044453428339323905384638363699366436503636357535673516346233993298316330843021297028227452732697246523622362231322812216213206020001921763154815181495141713801147114310901048

    // assert_eq!(largest_number(vec![
    //     4704, 6306, 9385, 7536, 3462, 4798, 5422, 5529, 8070, 6241, 9094, 7846, 663, 6221, 216,
    //     6758, 8353, 3650, 3836, 8183, 3516, 5909, 6744, 1548, 5712, 2281, 3664, 7100, 6698, 7321,
    //     4980, 8937, 3163, 5784, 3298, 9890, 1090, 7605, 1380, 1147, 1495, 3699, 9448, 5208, 9456,
    //     3846, 3567, 6856, 2000, 3575, 7205, 2697, 5972, 7471, 1763, 1143, 1417, 6038, 2313, 6554,
    //     9026, 8107, 9827, 7982, 9685, 3905, 8939, 1048, 282, 7423, 6327, 2970, 4453, 5460, 3399,
    //     9533, 914, 3932, 192, 3084, 6806, 273, 4283, 2060, 5682, 2, 2362, 4812, 7032, 810, 2465,
    //     6511, 213, 2362, 3021, 2745, 3636, 6265, 1518, 8398
    // ]), String::from("98909827968595339456944893859149094902689398937839883538183810810780707982784676057536747174237321720571007032685668066758674466986636554651163276306626562416221603859725909578457125682552954605422520849804812479847044453428339323905384638363699366436503636357535673516346233993298316330843021297028227452732697246523622362231322812216213206020001921763154815181495141713801147114310901048"));

    let mut a = vec![824, 938, 1399, 5607, 6973, 5703, 9609, 4398, 8247];
    dbg!(largest_number(a));

    a = vec![2, 2281];
    dbg!(largest_number(a));

    // which_bit(String::from("98909827968595339456944893859149094902689398937839883538183810810780707982784676057536747174237321720571007032685668066758674466986636554651163276306626562416221603859725909578457125682552954605422520849804812479847044453428339323905384638363699366436503636357535673516346233993298316330843021297028227452732697246523622362231322812216213206020001921763154815181495141713801147114310901048"),
    //           String::from("98909827968595339456944893859149094902689398937839883538183810781080707982784676057536747174237321720571007032685668066758674466986636554651163276306626562416221603859725909578457125682552954605422520849804812479847044453428339323905384638363699366436503636357535673516346233993298316330843021297028227452732697246523622362231322812216213206020001921763154815181495141713801147114310901048"));
}
