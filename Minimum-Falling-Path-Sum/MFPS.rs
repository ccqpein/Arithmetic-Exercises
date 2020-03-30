pub fn min_falling_path_sum(a: Vec<Vec<i32>>) -> i32 {
    use std::cmp::{max, min};
    let leng = a.len();
    let mut a_iter = a.into_iter();
    let mut temp = a_iter.next().unwrap();
    loop {
        if let Some(this_row) = a_iter.next() {
            let mut cache = vec![];
            for (ind, v) in this_row.iter().enumerate() {
                //println!("{}", ind - 1);
                let a = temp[max(ind as i32 - 1, 0) as usize];
                let b = temp[ind];
                let c = temp[min(ind as i32 + 1, leng as i32 - 1) as usize];
                let mm = min(min(a, b), c);
                cache.push(v + mm);
            }
            temp = cache;
        } else {
            break;
        }
    }
    *temp.iter().min().unwrap()
}

fn main() {
    dbg!(min_falling_path_sum(vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ]));
}
