pub fn smallest_range_ii(a: Vec<i32>, k: i32) -> i32 {
    if a.len() < 2 {
        return 0;
    }

    let mut a = a.clone();
    a.sort();

    let (mi, ma) = (a[0], a[a.len() - 1]);

    let mut dis = ma - mi;

    for ind in 0..a.len() - 1 {
        let (a, b) = (a[ind], a[ind + 1]);
        dis = *[
            dis,
            [ma - k, a + k].iter().max().unwrap() - [mi + k, b - k].iter().min().unwrap(),
        ]
        .iter()
        .min()
        .unwrap();
    }

    dis
}

fn main() {
    dbg!(smallest_range_ii(vec![1], 0));
    dbg!(smallest_range_ii(vec![0, 10], 2));
    dbg!(smallest_range_ii(vec![1, 3, 6], 3));
}
