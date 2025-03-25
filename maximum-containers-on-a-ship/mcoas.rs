// pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
//     let mut re = 0;
//     let mut count = 0;
//     for nn in 1..=(n * n) as usize {
//         re += w;
//         count += 1;
//         if re > max_weight {
//             return count - 1;
//         }
//     }
//     count
// }

pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
    (max_weight / w).min(n * n)
}

fn main() {}
