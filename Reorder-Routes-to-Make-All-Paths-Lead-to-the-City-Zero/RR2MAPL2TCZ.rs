use std::collections::{HashMap, HashSet};

pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let mut set: HashSet<(i32, i32)> = connections.iter().map(|x| (x[0], x[1])).collect();
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

    for road in connections {
        graph.entry(road[0]).or_insert(vec![]).push(road[1]);
        graph.entry(road[1]).or_insert(vec![]).push(road[0]);
    }

    dfs(0, -1, &graph, &set, &mut result)
}

fn dfs(
    i: i32,
    p: i32,
    graph: &HashMap<i32, Vec<i32>>,
    set: &HashSet<(i32, i32)>,
    result: &mut i32,
) -> i32 {
    match set.get(&(p, i)) {
        Some(_) => *result += 1,
        None => (),
    }

    for v in graph.get(&i).unwrap() {
        if *v == p {
            continue;
        }
        dfs(*v, i, graph, set, result);
    }

    *result
}

fn main() {
    assert_eq!(
        min_reorder(
            6,
            vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]
        ),
        3
    );

    assert_eq!(
        min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]),
        2
    );
}
