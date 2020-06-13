fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    candidates.sort();
    let mut list_of_sets = (0..=target)
        .map(|_| HashSet::new())
        .collect::<Vec<HashSet<Vec<i32>>>>();

    list_of_sets[0] = {
        let mut init = HashSet::new();
        init.insert(vec![]);
        init
    };

    for can in candidates {
        for t in (can..=target).rev() {
            let prevs = {
                list_of_sets[(t - can) as usize]
                    .iter()
                    .map(|x| x.clone())
                    .collect::<Vec<Vec<i32>>>()
            };

            for prev in prevs {
                let mut cache = prev;
                cache.push(can);
                list_of_sets[t as usize].insert(cache);
            }
        }
    }
    list_of_sets.pop().unwrap().into_iter().collect()
}

fn main() {
    assert_eq!(
        combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
        vec![vec![1, 7], vec![1, 2, 5], vec![2, 6], vec![1, 1, 6]]
    )
}
