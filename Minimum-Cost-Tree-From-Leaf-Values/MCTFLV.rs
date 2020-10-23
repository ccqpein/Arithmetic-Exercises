use std::collections::HashMap;
pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
    //mct_inner_help(&arr, 0, arr.len()).1
    let mut record = HashMap::new();
    mct_inner_help(&arr, 0, arr.len(), &mut record).1
}

// return largest element and non-leaf node
// fn mct_inner_help(a: &Vec<i32>, start_ind: usize, length: usize) -> (i32, i32) {
//     if start_ind == length - 2 {
//         // println!(
//         //     "start_ind: {}, largest: {}, node: {}",
//         //     start_ind,
//         //     max(a[start_ind], a[start_ind + 1]),
//         //     a[start_ind] * a[start_ind + 1]
//         // );
//         return (
//             max(a[start_ind], a[start_ind + 1]),
//             a[start_ind] * a[start_ind + 1],
//         );
//     }

//     if start_ind == length - 1 {
//         //println!("end: {}", a[start_ind]);
//         return (a[start_ind], 0);
//     }

//     if start_ind >= length {
//         return (0, 0);
//     }

//     use std::cmp::max;
//     let (largest0, node0) = mct_inner_help(a, start_ind + 1, length);
//     let a0 = a[start_ind] * largest0 + node0;

//     let (largest1, node1) = mct_inner_help(a, start_ind + 2, length);
//     let a1 =
//         max(a[start_ind], a[start_ind + 1]) * largest1 + a[start_ind] * a[start_ind + 1] + node1;

//     //println!("start_ind: {}, a0: {} a1: {}", start_ind, a0, a1);

//     if a0 < a1 {
//         (max(largest0, a[start_ind]), a0)
//     } else if a1 < a0 {
//         (
//             *[largest1, a[start_ind], a[start_ind + 1]]
//                 .iter()
//                 .max()
//                 .unwrap(),
//             a1,
//         )
//     } else {
//         (
//             *[largest0, a[start_ind], a[start_ind + 1], largest1]
//                 .iter()
//                 .max()
//                 .unwrap(),
//             a0,
//         )
//     }
// }

// fn mct_inner_help(a: &Vec<i32>, start_ind: usize, end_ind: usize) -> (i32, i32) {
//     use std::cmp::max;
//     match end_ind - start_ind {
//         0 => (0, 0),
//         1 => (a[start_ind], 0),
//         2 => (
//             max(a[start_ind], a[start_ind + 1]),
//             a[start_ind] * a[start_ind + 1],
//         ),
//         _ => (1..end_ind - start_ind)
//             .map(|i| {
//                 let (largest0, node0) = mct_inner_help(a, start_ind, start_ind + i);
//                 let (largest1, node1) = mct_inner_help(a, start_ind + i, end_ind);

//                 // println!(
//                 //     "indexs: {},{}; 0: {}, {}; 1: {}, {};",
//                 //     start_ind + i,
//                 //     end_ind,
//                 //     largest0,
//                 //     node0,
//                 //     largest1,
//                 //     node1
//                 // );
//                 (max(largest0, largest1), largest0 * largest1 + node0 + node1)
//             })
//             .min_by(|x, y| x.1.cmp(&y.1))
//             .unwrap(),
//     }
// }

fn mct_inner_help(
    a: &Vec<i32>,
    start_ind: usize,
    end_ind: usize,
    record: &mut HashMap<(usize, usize), (i32, i32)>,
) -> (i32, i32) {
    if let Some(a) = record.get(&(start_ind, end_ind)) {
        return *a;
    }
    use std::cmp::max;
    match end_ind - start_ind {
        0 => (0, 0),
        1 => {
            record.insert((start_ind, end_ind), (a[start_ind], 0));
            (a[start_ind], 0)
        }
        2 => {
            let x = max(a[start_ind], a[start_ind + 1]);
            let y = a[start_ind] * a[start_ind + 1];
            record.insert((start_ind, end_ind), (x, y));
            (x, y)
        }
        _ => {
            let (x, y) = (1..end_ind - start_ind)
                .map(|i| {
                    let (largest0, node0) = mct_inner_help(a, start_ind, start_ind + i, record);
                    let (largest1, node1) = mct_inner_help(a, start_ind + i, end_ind, record);

                    let x = max(largest0, largest1);
                    let y = largest0 * largest1 + node0 + node1;
                    (x, y)
                    //(max(largest0, largest1), largest0 * largest1 + node0 + node1)
                })
                .min_by(|x, y| x.1.cmp(&y.1))
                .unwrap();

            record.insert((start_ind, end_ind), (x, y));
            (x, y)
        }
    }
}

fn main() {
    assert_eq!(32, mct_from_leaf_values(vec![6, 2, 4]));
    assert_eq!(28, mct_from_leaf_values(vec![2, 4, 5]));
    assert_eq!(58, mct_from_leaf_values(vec![6, 2, 4, 5]));
    assert_eq!(100, mct_from_leaf_values(vec![6, 2, 4, 5, 7]));
    assert_eq!(156, mct_from_leaf_values(vec![6, 2, 4, 5, 7, 8]));
    assert_eq!(
        1166,
        mct_from_leaf_values(vec![
            5, 1, 2, 3, 15, 7, 3, 2, 2, 5, 9, 1, 11, 9, 15, 14, 7, 1, 5
        ])
    );
}
