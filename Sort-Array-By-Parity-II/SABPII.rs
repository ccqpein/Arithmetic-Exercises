pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
    let mut aa = a.clone();
    aa.sort();
    let mut even = aa.iter().filter(|&x| *x % 2 == 0);
    let mut odd = aa.iter().filter(|&x| *x % 2 != 0);

    let mut result = vec![];
    for _ in 0..a.len() / 2 {
        result.push(*even.next().unwrap());
        result.push(*odd.next().unwrap());
    }
    result
}

fn main() {
    dbg!(sort_array_by_parity_ii(vec![4, 2, 5, 7]));
}
