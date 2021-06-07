pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    //helper(&weights, days).unwrap()
    helper2(&weights, days)
}

// beyond time limit
fn helper(w: &[i32], days: i32) -> Option<i32> {
    if w.len() == 0 {
        return None;
    }

    if days == 1 {
        return Some(w.iter().sum());
    }

    let mut cache = vec![];
    for i in 0..w.len() {
        let a = w[0..i].iter().sum();
        let rest = helper(&w[i..], days - 1);
        if let None = rest {
            continue;
        }

        if a >= rest.unwrap() {
            cache.push(a);
        } else {
            cache.push(rest.unwrap())
        }
    }

    Some(*cache.iter().min().unwrap())
}

fn helper2(w: &[i32], days: i32) -> i32 {
    let (mut left, mut right) = (*w.iter().max().unwrap(), w.iter().sum::<i32>());
    while left < right {
        let (mid, mut need, mut cur) = ((left + right) / 2, 1, 0);
        for v in w {
            if cur + v > mid {
                need += 1;
                cur = 0;
            }
            cur += v
        }
        if need > days {
            left = mid + 1;
        } else {
            right = mid
        }
    }
    left
}

fn main() {
    assert_eq!(ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
    assert_eq!(ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
    assert_eq!(ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);

    dbg!(ship_within_days(
        vec![
            180, 373, 75, 82, 497, 23, 303, 299, 53, 426, 152, 314, 206, 433, 283, 370, 179, 254,
            265, 431, 453, 17, 189, 224
        ],
        12
    ));
}
