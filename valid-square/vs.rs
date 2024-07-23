fn distance(a: &Vec<i32>, b: &Vec<i32>) -> f64 {
    ((a[0] as f64 - b[0] as f64).powi(2) + (a[1] as f64 - b[1] as f64).powi(2)).sqrt()
}

pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
    let mut dists = [
        distance(&p1, &p2),
        distance(&p1, &p3),
        distance(&p1, &p4),
        distance(&p2, &p3),
        distance(&p2, &p4),
        distance(&p3, &p4),
    ];

    dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    //dbg!(dists);
    (dists[0] != 0.0)
        && (dists[0] == dists[1])
        && (dists[1] == dists[2])
        && (dists[2] == dists[3])
        && (dists[4] == dists[5])
}

fn main() {
    assert!(valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]));
    assert!(valid_square(
        vec![1, 0],
        vec![-1, 0],
        vec![0, 1],
        vec![0, -1]
    ));

    assert!(!valid_square(
        vec![0, 0],
        vec![0, 0],
        vec![0, 0],
        vec![0, 0]
    ));
}
