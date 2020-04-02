pub fn single_number(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut tbl: HashMap<i32, usize> = HashMap::new();
    for num in nums {
        *tbl.entry(num).or_insert(0) += 1;
    }
    *tbl.iter().filter(|x| *x.1 == 1).next().unwrap().0
}

fn main() {
    dbg!(single_number(vec![1, 1, 2]));
}
