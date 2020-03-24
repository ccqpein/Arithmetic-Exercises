// fn make_squares_list(n: i32) -> Vec<i32> {
//     let mut re: Vec<i32> = vec![];
//     for i in 0..n + 1 {
//         let temp = i * i;
//         if temp > n {
//             break;
//         } else {
//             re.push(temp);
//         }
//     }

//     re
// }

// fn sum_list(l1: &Vec<i32>, l2: &Vec<i32>) -> Vec<i32> {
//     let mut temp: Vec<Vec<i32>> = l1
//         .iter()
//         .map(|a| l2.iter().map(|b| b + a).collect())
//         .collect();

//     let mut re: Vec<i32> = vec![];
//     for i in temp.iter_mut() {
//         re.append(i);
//     }

//     re
// }

// fn gen_all_nums(time: &mut u32, basic: &Vec<i32>) -> Vec<i32> {
//     let mut re: Vec<i32> = sum_list(basic, basic);
//     while (*time > 2) {
//         let temp = re.clone();
//         re.append(&mut sum_list(basic, &temp));
//         *time -= 1;
//         //println!("{:?}", re);
//     }
//     re
// }

// fn ps(n: i32) -> i32 {
//     let basic_list = make_squares_list(n);

//     for i in 2..4 {
//         if gen_all_nums(&mut (i as u32), &basic_list).contains(&n) {
//             return i;
//         }
//     }

//     0
// }

pub fn num_squares(n: i32) -> i32 {
    let mut temp = Vec::new();
    for i in 1..=n {
        if i * i == n {
            return 1;
        }
        temp.push(i * i)
    }

    let one = temp.clone();
    let mut flag = 1;
    let mut cache = Vec::new();

    loop {
        for i in temp {
            for o in &one {
                if i + o == n {
                    return flag + 1;
                } else if i + o < n {
                    cache.push(i + o);
                } else {
                    break;
                }
            }
        }
        temp = cache;
        cache = Vec::new();
        flag += 1;
    }
}

fn main() {
    println!("{:?}", num_squares(13)); //=> 2
    println!("{:?}", num_squares(3)); //=> 3
    println!("{:?}", num_squares(12)); //=> 3
}
