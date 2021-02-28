pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut cache = vec![0; 1000];
    trips.iter().for_each(|v| {
        cache[v[1] as usize] += v[0];
        cache[v[2] as usize] -= v[0]
    });

    //dbg!(&cache[0..10]);

    let mut a = 0;
    for v in cache {
        a += v;
        if a > capacity {
            return false;
        }
    }
    true
}

fn main() {
    assert!(!car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4));
    assert!(car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5));
    assert!(car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7]], 3));
    assert!(car_pooling(
        vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]],
        11
    ));
}
