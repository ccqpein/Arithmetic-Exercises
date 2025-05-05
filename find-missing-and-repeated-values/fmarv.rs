pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut cache = vec![0; grid.len() * grid.len()];
    let mut res = vec![0, 0];
    for l in grid {
        for i in l {
            if cache[i as usize - 1] == 1 {
                res[0] = i;
            } else {
                cache[i as usize - 1] += 1;
            }
        }
    }
    let (ind, _) = cache
        .into_iter()
        .enumerate()
        .find(|(_ind, v)| *v == 0)
        .unwrap();
    res[1] = (ind + 1) as i32;
    res
}

fn main() {
    dbg!(find_missing_and_repeated_values(vec![
        vec![1, 3],
        vec![2, 2]
    ]));

    dbg!(find_missing_and_repeated_values(vec![
        vec![9, 1, 7],
        vec![8, 9, 2],
        vec![3, 4, 6],
    ]));
}
