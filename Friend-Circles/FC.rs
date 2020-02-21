use std::collections::HashSet;

pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
    let mut set: HashSet<(usize, usize)> = HashSet::new();

    let row_num = m.len();
    let col_num = m[0].len();

    let mut result = 0;

    for r in 0..row_num {
        for c in 0..col_num {
            if !set.contains(&(r, c)) && m[r][c] != 0 {
                result += 1;
                println!("{:?}", &(r, c));
                search(&m, &mut set, &(r, c));
            }
        }
    }
    result
}

fn search(m: &Vec<Vec<i32>>, set: &mut HashSet<(usize, usize)>, coop: &(usize, usize)) {
    if set.contains(coop) {
        return;
    }
    set.insert(coop.clone());
    if m[coop.0][coop.1] == 0 {
        return;
    }
    //println!("{:?}", coop);

    let coooo = vec![
        (coop.0 as i32 - 1, coop.1 as i32),
        (coop.0 as i32 + 1, coop.1 as i32),
        (coop.0 as i32, coop.1 as i32 - 1),
        (coop.0 as i32, coop.1 as i32 + 1),
    ];

    for new_coop in coooo {
        if new_coop.0 < 0
            || new_coop.0 >= m.len() as i32
            || new_coop.1 < 0
            || new_coop.1 >= m[0].len() as i32
        {
            continue;
        }

        let new_row = new_coop.0;
        let new_col = new_coop.1;
        search(m, set, &(new_row as usize, new_col as usize));
    }
}

fn main() {
    dbg!(find_circle_num(vec![
        vec![1, 1, 0],
        vec![1, 1, 0],
        vec![0, 0, 1]
    ])); // 2

    dbg!(find_circle_num(vec![
        vec![1, 1, 0],
        vec![1, 1, 1],
        vec![0, 1, 1]
    ])); // 1

    dbg!(find_circle_num(vec![
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 1]
    ])); // 3

    dbg!(find_circle_num(vec![
        vec![1, 0, 0, 1],
        vec![0, 1, 1, 0],
        vec![0, 1, 1, 1],
        vec![1, 0, 1, 1]
    ])); // 1
}
