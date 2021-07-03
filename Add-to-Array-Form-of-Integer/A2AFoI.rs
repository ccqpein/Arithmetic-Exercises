// pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
//     let mut l = num.len() as i32;
//     let mut sum = num.iter().fold(0, |x, y| {
//         l -= 1;
//         x + y * 10_f64.powi(l) as i32
//     }) + k;
//     //dbg!(sum);
//     let mut result = vec![];
//     loop {
//         dbg!(sum);
//         result.push(sum % 10);
//         if sum % 10 == sum {
//             break;
//         } else {
//             sum /= 10;
//         }
//     }

//     result.reverse();
//     result
// }

pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
    let mut kl = vec![];
    loop {
        kl.push(k % 10);
        if k % 10 == k {
            break;
        } else {
            k /= 10;
        }
    }

    num.reverse();

    let mut a = 0;
    let mut result = vec![];
    let mut ind = 0;
    //println!("num: {:?}, kl:{:?}", num, kl);

    loop {
        dbg!(&result);
        dbg!(&ind);
        match (num.get(ind), kl.get(ind)) {
            (Some(n), Some(k)) => {
                let cache = n + k + a;
                if cache >= 10 {
                    result.push(cache % 10);
                    a = cache / 10;
                } else {
                    result.push(cache);
                    a = 0
                }
            }
            (Some(n), None) => {
                println!("here?");
                if a != 0 {
                    let cache = n + a;
                    if cache >= 10 {
                        result.push(cache % 10);
                        a = cache / 10;
                    } else {
                        result.push(cache);
                        result.extend_from_slice(&num[ind + 1..num.len()]);
                        break;
                    }
                } else {
                    result.extend_from_slice(&num[ind..num.len()]);
                    break;
                }
            }
            (None, Some(n)) => {
                if a != 0 {
                    let cache = n + a;
                    if cache >= 10 {
                        result.push(cache % 10);
                        a = cache / 10;
                    } else {
                        result.push(cache);
                        result.extend_from_slice(&kl[ind + 1..kl.len()]);
                        break;
                    }
                } else {
                    result.extend_from_slice(&kl[ind..kl.len()]);
                    break;
                }
            }
            _ => {
                // none, none
                if a != 0 {
                    result.push(a);
                }
                break;
            }
        }
        ind += 1;
    }

    result.reverse();
    result
}

fn main() {
    assert_eq!(add_to_array_form(vec![1, 2, 0, 0], 34), vec![1, 2, 3, 4]);
    assert_eq!(add_to_array_form(vec![2, 1, 5], 806), vec![1, 0, 2, 1]);
    assert_eq!(add_to_array_form(vec![2, 7, 4], 181), vec![4, 5, 5]);
    assert_eq!(
        add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1),
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(add_to_array_form(vec![0], 23), vec![2, 3]);
    assert_eq!(add_to_array_form(vec![6], 809), vec![8, 1, 5]);
}
