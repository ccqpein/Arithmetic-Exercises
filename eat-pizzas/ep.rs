// pub fn max_weight(mut pizzas: Vec<i32>) -> i64 {
//     use std::cmp::max;
//     pizzas.sort_by(|a, b| b.cmp(a));
//     dbg!(&pizzas);
//     let steps = pizzas.len() / 4;
//     let mut result0 = 0;
//     let mut jumps = [2, 1].repeat(steps);
//     jumps.reverse();
//     let mut ind = 0;
//     for _ in 0..steps {
//         result0 += pizzas[ind] as i64;
//         ind += jumps.pop().unwrap()
//     }

//     let mut result1 = 0;
//     let mut jumps = [1, 2].repeat(steps);
//     jumps.reverse();
//     ind = 1;
//     for _ in 0..steps {
//         result1 += pizzas[ind] as i64;
//         ind += jumps.pop().unwrap()
//     }

//     max(result0, result1)
// }

pub fn max_weight(mut pizzas: Vec<i32>) -> i64 {
    pizzas.sort_by(|a, b| b.cmp(a));
    //dbg!(&pizzas);
    let steps = pizzas.len() / 4;
    let mut result: i64 = 0;

    let xx = steps / 2 + steps % 2;
    let yy = steps / 2;
    for ind in 0..xx {
        result += pizzas[ind] as i64;
    }

    let rest = pizzas.get(xx..).unwrap();
    for ind in 0..yy {
        result += rest[ind * 2 + 1] as i64
    }
    result
}

fn main() {
    assert_eq!(14, max_weight(vec![1, 2, 3, 4, 5, 6, 7, 8]));
    assert_eq!(3, max_weight(vec![2, 1, 1, 1, 1, 1, 1, 1]));

    assert_eq!(14, max_weight(vec![5, 2, 2, 4, 3, 3, 1, 3, 2, 5, 4, 2]));
}
