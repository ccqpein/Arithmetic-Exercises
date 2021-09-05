use std::collections::HashMap;

pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    let mut table = HashMap::new();

    for num in arr {
        let m = num.rem_euclid(k);
        let cm = (-num).rem_euclid(k);

        //dbg!(&m, &cm);

        if let None = table.get(&cm) {
            let en = table.entry(m).or_insert(0);
            *en += 1;
        } else {
            let en = table.get_mut(&cm).unwrap();
            *en -= 1;
            if *en == 0 {
                table.remove(&cm);
            }
        }
    }

    //dbg!(&table);

    if table.len() == 0 {
        true
    } else {
        false
    }
}

fn main() {
    assert!(can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5));
    assert!(can_arrange(vec![1, 2, 3, 4, 5, 6], 7));
    assert!(!can_arrange(vec![1, 2, 3, 4, 5, 6], 10));
    assert!(can_arrange(vec![10, -10], 2));
    assert!(can_arrange(vec![-1, 1, -2, 2, -3, 3, -4, 4], 3));
}
