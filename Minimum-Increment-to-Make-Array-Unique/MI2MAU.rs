pub fn min_increment_for_unique(mut a: Vec<i32>) -> i32 {
    a.sort();

    let mut result = 0;
    let mut largest = if let Some(u) = a.last() { *u } else { return 0 };
    let mut k = a[0];

    let mut option_results = vec![];
    {
        let mut o_v = 1;
        for &v in &a {
            if v - o_v > 1 {
                option_results.append(&mut (o_v + 1..v).into_iter().collect::<Vec<_>>());
            }
            o_v = v
        }
    };

    println!("{:?}, {:?}", option_results, a);

    for i in 1..a.len() {
        if a[i] == k {
            while option_results.len() != 0 && option_results[0] <= k {
                option_results = option_results.drain(1..).collect();
            }
            if option_results.len() != 0 {
                result += option_results[0] - k;
                option_results = option_results.drain(1..).collect();
            } else {
                result += 1 + largest - k;
                largest += 1
            }
        } else {
            k = a[i];
        }
    }
    result
}

fn main() {
    let test0 = vec![1, 2, 2];
    let test1 = vec![3, 2, 1, 2, 1, 7];

    assert_eq!(min_increment_for_unique(test0), 1);
    assert_eq!(min_increment_for_unique(test1), 6);
    assert_eq!(min_increment_for_unique(vec![]), 0);
    assert_eq!(min_increment_for_unique(vec![0, 2, 2]), 1);
    assert_eq!(min_increment_for_unique(vec![1, 2, 2, 2]), 3);
}
