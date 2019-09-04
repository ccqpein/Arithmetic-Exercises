pub fn num_trees(n: i32) -> i32 {
    use std::collections::HashMap;
    let mut m = HashMap::new();
    m.insert(0, 1);
    m.insert(1, 1);
    for i in 2..(n + 1) as usize {
        for j in 0..i {
            let temp = m.get(&i).unwrap_or(&0).clone();
            m.insert(
                i,
                *m.get(&j).unwrap_or(&0) * *m.get(&(i - j - 1)).unwrap_or(&0) + temp,
            );
        }
    }

    dbg!(&m);

    *m.get(&(n as usize)).unwrap() as i32
}

fn main() {
    dbg!(num_trees(3));
}
