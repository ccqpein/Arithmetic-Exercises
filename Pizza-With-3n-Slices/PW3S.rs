/*pub fn max_size_slices(slices: Vec<i32>) -> i32 {
    use std::iter::FromIterator;

    if slices.len() == 3 {
        return *slices.iter().max().unwrap();
    }
    let mut result = 0;
    for (ind, &ele) in slices.iter().enumerate() {
        if ele > result {
            result = ele;
        }

        let (start, end);
        let new_slices = if ind == 0 {
            start = 2;
            end = slices.len() - 1;
            Vec::from_iter(slices[start..end].iter().cloned())
        } else if ind == slices.len() - 1 {
            start = 1;
            end = slices.len() - 1 - 1;
            Vec::from_iter(slices[start..end].iter().cloned())
        } else {
            let mut cache = Vec::from_iter(slices[ind + 2..].iter().cloned());
            cache.append(&mut Vec::from_iter(slices[0..ind - 1].iter().cloned()));
            cache
        };

        if new_slices.len() != 0 {
            let cache = ele + max_size_slices(new_slices);
            if cache > result {
                result = cache
            }
        } else {
            if ele > result {
                result = ele
            }
        }
    }
    result
}*/

/*pub fn max_size_slices(slices: Vec<i32>) -> i32 {
    max_inner(&slices, slices.len() / 3)
}

pub fn max_inner(slices: &Vec<i32>, left: usize) -> i32 {
    use std::iter::FromIterator;

    if left == 0 {
        return 0;
    }
    if slices.len() == 3 {
        return *slices.iter().max().unwrap();
    }
    let mut result = 0;
    for (ind, &ele) in slices.iter().enumerate() {
        if ele > result {
            result = ele;
        }

        let (start, end);
        let new_slices = if ind == 0 {
            start = 2;
            end = slices.len() - 1;
            Vec::from_iter(slices[start..end].iter().cloned())
        } else if ind == slices.len() - 1 {
            start = 1;
            end = slices.len() - 1 - 1;
            Vec::from_iter(slices[start..end].iter().cloned())
        } else {
            let mut cache = Vec::from_iter(slices[ind + 2..].iter().cloned());
            cache.append(&mut Vec::from_iter(slices[0..ind - 1].iter().cloned()));
            cache
        };

        if new_slices.len() != 0 {
            let cache = ele + max_inner(&new_slices, left - 1);
            if cache > result {
                result = cache
            }
        } else {
            if ele > result {
                result = ele
            }
        }
    }
    result
}*/

pub fn max_size_slices(slices: Vec<i32>) -> i32 {
    max_inner(&slices, slices.len() / 3)
}

pub fn max_inner(slices: &Vec<i32>, left: usize) -> i32 {
    if slices.len() <= 3 || left == 1 {
        return *slices.iter().max().unwrap();
    }

    let mut cache =
        vec![max_inner(&slices[1..slices.len() - 2].to_vec(), left - 1) + slices.last().unwrap()];

    //println!("fake len: {}, level: {}", fake_len, level);
    cache.append(
        &mut (0..=slices.len() - 3)
            .map(|x| {
                let end_index = if x >= 1 {
                    slices.len()
                } else {
                    slices.len() - 1
                };
                max_inner(&slices[x + 2..end_index].to_vec(), left - 1) + slices[x]
            })
            .collect::<Vec<i32>>(),
    );

    *cache.iter().max().unwrap()
}

fn main() {
    assert_eq!(max_size_slices(vec![3, 1, 2]), 3);
    assert_eq!(max_size_slices(vec![4, 1, 2, 5, 8, 3, 1, 9, 7]), 21);

    assert_eq!(max_size_slices(vec![8, 9, 8, 6, 1, 1]), 16);
    assert_eq!(max_size_slices(vec![1, 2, 3, 4, 5, 6]), 10);

    assert_eq!(max_size_slices(vec![10, 9, 1, 10, 8, 5, 10, 2, 2]), 30);
    assert_eq!(
        max_size_slices(vec![9, 8, 1, 7, 7, 9, 5, 10, 7, 9, 3, 8, 3, 4, 8]),
        45
    );

    assert_eq!(
        max_size_slices(vec![9, 5, 1, 7, 8, 4, 4, 5, 5, 8, 7, 7]), // 9 + 8 + 5 + 8
        30
    );

    assert_eq!(
        max_size_slices(vec![
            2, 4, 3, 1, 10, 6, 1, 2, 10, 2, 8, 9, 4, 8, 8, 8, 10, 3, 9, 10, 7, 9, 4, 5, 4, 3, 1,
            10, 6, 1
        ]),
        85
    );

    dbg!(max_size_slices(vec![
        6, 3, 1, 2, 6, 2, 4, 3, 10, 4, 1, 4, 6, 5, 5, 3, 4, 7, 6, 5, 8, 7, 3, 8, 8, 1, 7, 1, 7, 8
    ]));
}
