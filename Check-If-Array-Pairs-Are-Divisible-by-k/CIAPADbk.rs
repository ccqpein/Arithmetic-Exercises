use std::collections::HashMap;

pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    if arr.len() == 2 {
        return arr.iter().all(|v| v % k == 0);
    }

    let arr = arr.into_iter().map(|v| v % k).collect::<Vec<i32>>();

    dbg!(&arr);

    let mut count_table: HashMap<i32, usize> = HashMap::new();

    arr.iter().for_each(|v| {
        let en = count_table.entry(*v).or_insert(0);
        *en += 1;
    });

    dbg!(&count_table);

    for &kk in count_table.keys() {
        if kk == 0 {
            continue;
        }

        let count = if let Some(count) = count_table.get(&kk) {
            count
        } else {
            return false;
        };

        let pair_count = if let Some(count) = count_table.get(&(k - kk)) {
            count
        } else {
            return false;
        };

        if *pair_count != *count {
            return false;
        }
    }

    true
}

fn main() {
    assert!(can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5));
    assert!(can_arrange(vec![1, 2, 3, 4, 5, 6], 7));
    assert!(!can_arrange(vec![1, 2, 3, 4, 5, 6], 10));
    assert!(can_arrange(vec![10, -10], 2));
    assert!(can_arrange(vec![-1, 1, -2, 2, -3, 3, -4, 4], 3));
}
