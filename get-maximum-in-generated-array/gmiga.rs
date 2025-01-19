use std::collections::HashMap;

pub fn get_maximum_generated(n: i32) -> i32 {
    let mut table = HashMap::new();
    (0..=n)
        .into_iter()
        .map(|i| helper(i, &mut table))
        .max()
        .unwrap()
}

fn helper(n: i32, table: &mut HashMap<i32, i32>) -> i32 {
    if let Some(v) = table.get(&n) {
        return *v;
    } else {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                if n % 2 == 0 {
                    let a = helper(n / 2, table);
                    table.insert(n, a);
                    a
                } else {
                    let a = helper(n / 2, table) + helper(n / 2 + 1, table);
                    table.insert(n, a);
                    a
                }
            }
        }
    }
}

fn main() {
    dbg!(get_maximum_generated(7));
    dbg!(get_maximum_generated(2));
    dbg!(get_maximum_generated(3));
    dbg!(get_maximum_generated(4));
}
