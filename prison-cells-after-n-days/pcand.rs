pub fn prison_after_n_days(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut cache = HashMap::new();
    let mut cache2: HashMap<i32, Vec<i32>> = HashMap::new();

    for nn in 1..=n {
        cells = (0..cells.len())
            .map(|ind| match ind {
                0 => 0,
                n if n == cells.len() - 1 => 0,
                n @ _ => {
                    if cells[n - 1] == cells[n + 1] {
                        1
                    } else {
                        0
                    }
                }
            })
            .collect();

        if let Some(nnn) = cache.get(&cells) {
            //dbg!(nnn);
            return cache2
                .get(&(((n - nnn) % (nn - nnn)) + nnn))
                .unwrap()
                .to_vec();
        }

        cache.insert(cells.clone(), nn);
        cache2.insert(nn, cells.clone());
    }
    cells
}

fn main() {
    dbg!(prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7));
    dbg!(prison_after_n_days(
        vec![1, 0, 0, 1, 0, 0, 1, 0],
        1000000000
    ));
}
