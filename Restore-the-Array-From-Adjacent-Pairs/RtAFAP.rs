pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::{HashMap, HashSet};
    let len = adjacent_pairs.len();
    let mut table: HashMap<i32, Vec<i32>> = HashMap::with_capacity(len + 1);
    let mut set: HashSet<i32> = HashSet::with_capacity(len + 1);

    for p in adjacent_pairs {
        let x = p[0];
        let y = p[1];

        let en = table.entry(x).or_insert(Vec::with_capacity(2));
        if !en.contains(&y) {
            en.push(y);
        }

        if en.len() == 1 {
            set.insert(x);
        } else {
            set.remove(&x);
        }

        let en = table.entry(y).or_insert(Vec::with_capacity(2));
        if !en.contains(&x) {
            en.push(x);
        }

        if en.len() == 1 {
            set.insert(y);
        } else {
            set.remove(&y);
        }
    }

    let mut result = Vec::with_capacity(len + 1);
    let mut last = 100001;
    let mut start_p = *set.iter().next().unwrap();
    result.push(start_p);
    loop {
        let a = table
            .get(&start_p)
            .unwrap()
            .iter()
            .filter(|x| **x != last)
            .next()
            .unwrap();
        result.push(*a);
        if result.len() == len + 1 {
            break;
        }
        last = start_p;
        start_p = *a;
    }

    result
}

fn main() {
    dbg!(restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]]));
    dbg!(restore_array(vec![vec![4, -2], vec![1, 4], vec![-3, 1]]));
    dbg!(restore_array(vec![vec![100000, -100000]]));
}
