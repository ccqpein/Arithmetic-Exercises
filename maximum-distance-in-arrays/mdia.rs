pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let mut left_edges = vec![];
    let mut right_edges = vec![];
    for (ind, arr) in arrays.into_iter().enumerate() {
        left_edges.push((arr[0], ind));
        right_edges.push((arr[arr.len() - 1], ind));
    }

    left_edges.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    right_edges.sort_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());

    let mut res = 0;

    if right_edges[0].1 != left_edges[0].1 {
        res = res.max((right_edges[0].0 - left_edges[0].0).abs())
    }

    if right_edges[1].1 != left_edges[0].1 {
        res = res.max((right_edges[1].0 - left_edges[0].0).abs())
    }

    if right_edges[0].1 != left_edges[1].1 {
        res = res.max((right_edges[0].0 - left_edges[1].0).abs())
    }

    if right_edges[1].1 != left_edges[1].1 {
        res = res.max((right_edges[1].0 - left_edges[1].0).abs())
    }

    res
}

fn main() {
    dbg!(max_distance(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]]));
    dbg!(max_distance(vec![vec![1], vec![1]]));
}
