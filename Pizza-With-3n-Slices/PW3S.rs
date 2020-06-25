pub fn max_size_slices(slices: Vec<i32>) -> i32 {
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
}

fn main() {
    assert_eq!(max_size_slices(vec![3, 1, 2]), 3);
    assert_eq!(max_size_slices(vec![4, 1, 2, 5, 8, 3, 1, 9, 7]), 21);

    assert_eq!(max_size_slices(vec![8, 9, 8, 6, 1, 1]), 16);
    assert_eq!(max_size_slices(vec![1, 2, 3, 4, 5, 6]), 10);

    dbg!(max_size_slices(vec![
        6, 3, 1, 2, 6, 2, 4, 3, 10, 4, 1, 4, 6, 5, 5, 3, 4, 7, 6, 5, 8, 7, 3, 8, 8, 1, 7, 1, 7, 8
    ]));
}
