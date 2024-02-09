use std::collections::{HashMap, HashSet};

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut table = HashMap::new();
    edges.into_iter().for_each(|v| {
        table.entry(v[0]).or_insert(vec![]).push(v[1].clone());
        table.entry(v[1]).or_insert(vec![]).push(v[0]);
    });
    let mut paths = HashSet::new();
    let mut cache = HashMap::new();
    helper(
        &table,
        &mut cache,
        &mut paths,
        &source,
        &destination,
        n as usize,
    )
}

fn helper(
    table: &HashMap<i32, Vec<i32>>,
    cache: &mut HashMap<i32, bool>,
    paths: &mut HashSet<i32>,
    this: &i32,
    end: &i32,
    length: usize,
) -> bool {
    if cache.get(this).is_some() {
        return false;
    }

    paths.insert(*this);
    if this == end {
        true
    } else {
        table
            .get(this)
            .unwrap_or(&vec![])
            .iter()
            .find(|v| {
                if *v == end {
                    true
                } else {
                    if length == 0 {
                        paths.remove(this);
                        false
                    } else if paths.contains(v) {
                        paths.remove(this);
                        false
                    } else {
                        let x = helper(table, cache, paths, v, end, length - 1);
                        cache.insert(*this, x);
                        x
                    }
                }
            })
            .is_some()
    }
}

fn main() {
    assert!(valid_path(
        3,
        vec![vec![0, 1], vec![1, 2], vec![2, 0]],
        0,
        2
    ));

    assert!(!valid_path(
        6,
        vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
        0,
        5
    ));

    assert!(valid_path(1, vec![], 0, 0));

    assert!(!valid_path(
        50,
        vec![
            vec![31, 5],
            vec![10, 46],
            vec![19, 31],
            vec![5, 1],
            vec![31, 28],
            vec![28, 29],
            vec![8, 26],
            vec![13, 23],
            vec![16, 34],
            vec![30, 1],
            vec![16, 18],
            vec![33, 46],
            vec![27, 35],
            vec![2, 25],
            vec![49, 33],
            vec![44, 19],
            vec![22, 26],
            vec![30, 13],
            vec![27, 12],
            vec![8, 16],
            vec![42, 13],
            vec![18, 3],
            vec![21, 20],
            vec![2, 17],
            vec![5, 48],
            vec![41, 37],
            vec![39, 37],
            vec![2, 11],
            vec![20, 26],
            vec![19, 43],
            vec![45, 7],
            vec![0, 21],
            vec![44, 23],
            vec![2, 39],
            vec![27, 36],
            vec![41, 48],
            vec![17, 42],
            vec![40, 32],
            vec![2, 28],
            vec![35, 38],
            vec![3, 9],
            vec![41, 30],
            vec![5, 11],
            vec![24, 22],
            vec![39, 5],
            vec![40, 31],
            vec![18, 35],
            vec![23, 39],
            vec![20, 24],
            vec![45, 12]
        ],
        29,
        46
    ));
}
