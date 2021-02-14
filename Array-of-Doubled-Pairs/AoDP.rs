use std::collections::HashMap;

pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
    let mut table = HashMap::new();
    arr.into_iter().for_each(|v| {
        let entry = table.entry(v).or_insert(0);
        *entry += 1
    });

    let mut keys = table.keys().cloned().collect::<Vec<_>>();
    keys.sort_by(|a, b| match (*a < 0, *b < 0) {
        (true, true) => {
            if a < b {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        }
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.partial_cmp(&b).unwrap(),
    });

    //println!("keys: {:?}", keys);

    for k in keys {
        if *table.get(&k).unwrap() == 0 {
            continue;
        }

        let temp_k = k * 2;

        if let Some(v) = table.get(&temp_k) {
            let new_v = v - *table.get(&k).unwrap();
            if new_v < 0 {
                return false;
            }
            table.insert(temp_k, new_v);
        } else {
            return false;
        }
    }

    true
}

fn main() {
    assert!(!can_reorder_doubled(vec![3, 1, 3, 6]));
    assert!(!can_reorder_doubled(vec![2, 1, 2, 6]));
    assert!(can_reorder_doubled(vec![4, -2, 2, -4]));
    assert!(!can_reorder_doubled(vec![1, 2, 4, 16, 8, 4]));
    assert!(!can_reorder_doubled(vec![-5, -2]));
}
