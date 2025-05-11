pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res = vec![vec![]; num_rows as usize];
    res[0] = vec![1];
    for i in 1..num_rows as usize {
        res[i] = make_new(res.get(i - 1).unwrap());
    }
    res
}

fn make_new(row: &[i32]) -> Vec<i32> {
    let mut aa: Vec<i32> = row
        .windows(2)
        .fold(vec![1], |mut init, a| {
            init.push(a[0] + a[1]);
            init
        })
        .into_iter()
        .collect();
    aa.push(1);
    aa
}

fn main() {}
