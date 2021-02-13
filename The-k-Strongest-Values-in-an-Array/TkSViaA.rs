pub fn get_strongest(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
    arr.sort();
    let length = arr.len();

    let med = arr[{
        if length % 2 == 0 {
            (length - 1) / 2
        } else {
            length / 2
        }
    }];

    let mut cache = arr
        .iter()
        .enumerate()
        .map(|(ind, v)| (v - med, ind))
        .collect::<Vec<_>>();

    //println!("med: {:?}, arr: {:?}, cache: {:?}", med, arr, cache);

    cache.sort_by(|a, b|
                  //a.0.abs().partial_cmp(&b.0.abs()).unwrap()
                  {if a.0.abs() > b.0.abs() {
                      //std::cmp::Ordering::Greater
                      std::cmp::Ordering::Less
                  }else if a.0.abs() == b.0.abs() && a.0 > b.0{
                      //std::cmp::Ordering::Greater
                      std::cmp::Ordering::Less
                  }else {
                      //std::cmp::Ordering::Less
                      std::cmp::Ordering::Greater
                  }});

    //println!("med: {:?}, arr: {:?}, cache: {:?}", med, arr, cache);

    cache[0..k as usize]
        .iter()
        .map(|(_, ind)| arr[*ind])
        .collect()
}

fn main() {
    assert_eq!(get_strongest(vec![1, 2, 3, 4, 5], 2), vec![5, 1]);
    assert_eq!(get_strongest(vec![1, 1, 3, 5, 5], 2), vec![5, 5]);
    assert_eq!(get_strongest(vec![1, 1, 3, 5, 5], 2), vec![5, 5]);
    assert_eq!(
        get_strongest(vec![6, 7, 11, 7, 6, 8], 5),
        vec![11, 8, 6, 6, 7]
    );
    assert_eq!(get_strongest(vec![6, -3, 7, 2, 11], 3), vec![-3, 11, 2]);
    assert_eq!(get_strongest(vec![-7, 22, 17, 3], 2), vec![22, 17]);
}
