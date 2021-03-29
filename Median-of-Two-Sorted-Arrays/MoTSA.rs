// pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
//     let (l1, l2) = (nums1.len() as f32, nums2.len() as f32);

//     let l = ((l1 + l2) / 2_f32).floor() as usize;
//     dbg!(l);
//     if l == 0 {
//         if l1 > l2 {
//             return nums1[0] as f64;
//         } else {
//             return nums2[0] as f64;
//         }
//     }

//     let (mut n1, mut n2) = (nums1.iter(), nums2.iter());
//     let (mut a, mut b) = (n1.next(), n2.next());
//     let mut i = 0;
//     while i < l - 1 {
//         match (a, b) {
//             (Some(aa), Some(bb)) => {
//                 if aa >= bb {
//                     b = n2.next();
//                 } else {
//                     a = n1.next();
//                 }
//                 i += 1;
//             }
//             (None, Some(_)) => {
//                 a = n2.next();
//             }
//             (Some(_), None) => {
//                 b = n1.next();
//             }
//             _ => {}
//         }
//     }

//     match (a, b) {
//         (None, Some(_)) => {
//             a = n2.next();
//         }
//         (Some(_), None) => {
//             b = n1.next();
//         }
//         _ => {}
//     }

//     dbg!(a);
//     dbg!(b);

//     if (l1 + l2) % 2_f32 == 0_f32 {
//         return (*a.unwrap() as f64 + *b.unwrap() as f64) / 2_f64;
//     } else {
//         (*a.unwrap() as f64).max(*b.unwrap() as f64)
//     }
// }

pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    let (l1, l2) = (nums1.len() as f32, nums2.len() as f32);
    let l = ((l1 + l2) / 2_f32).floor() as usize;

    nums1.append(&mut nums2);
    nums1.sort();

    let mut n = nums1.iter();
    if (l1 + l2) % 2_f32 == 0_f32 {
        (*n.nth(l - 1).unwrap() as f64 + *n.next().unwrap() as f64) / 2_f64
    } else {
        *n.nth(l).unwrap() as f64
    }
}

fn main() {
    assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    assert_eq!(find_median_sorted_arrays(vec![0, 0], vec![0, 0]), 0.0);
    assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
    assert_eq!(find_median_sorted_arrays(vec![], vec![1, 2]), 1.5);
    assert_eq!(find_median_sorted_arrays(vec![2], vec![]), 2.0);
    assert_eq!(find_median_sorted_arrays(vec![3], vec![-2, -1]), -1.0);
}
