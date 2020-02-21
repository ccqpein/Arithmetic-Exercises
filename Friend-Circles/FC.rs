use std::collections::{HashMap, HashSet};

// pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
//     let mut set: HashSet<(usize, usize)> = HashSet::new();

//     let row_num = m.len();
//     let col_num = m[0].len();

//     let mut result = 0;

//     for r in 0..row_num {
//         for c in 0..col_num {
//             if !set.contains(&(r, c)) && m[r][c] != 0 {
//                 result += 1;
//                 println!("{:?}", &(r, c));
//                 search(&m, &mut set, &(r, c));
//             }
//         }
//     }
//     result
// }

// fn search(m: &Vec<Vec<i32>>, set: &mut HashSet<(usize, usize)>, coop: &(usize, usize)) {
//     if set.contains(coop) {
//         return;
//     }
//     set.insert(coop.clone());
//     if m[coop.0][coop.1] == 0 {
//         return;
//     }
//     //println!("{:?}", coop);

//     let coooo = vec![
//         (coop.0 as i32 - 1, coop.1 as i32),
//         (coop.0 as i32 + 1, coop.1 as i32),
//         (coop.0 as i32, coop.1 as i32 - 1),
//         (coop.0 as i32, coop.1 as i32 + 1),
//     ];

//     for new_coop in coooo {
//         if new_coop.0 < 0
//             || new_coop.0 >= m.len() as i32
//             || new_coop.1 < 0
//             || new_coop.1 >= m[0].len() as i32
//         {
//             continue;
//         }

//         let new_row = new_coop.0;
//         let new_col = new_coop.1;
//         search(m, set, &(new_row as usize, new_col as usize));
//     }
// }

pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
    let row_num = m.len();
    let col_num = m[0].len();

    let mut friends_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for r in 0..row_num {
        for c in 0..col_num {
            if m[r][c] == 1 {
                let content = friends_map.entry(r).or_insert(vec![]);
                content.push(c);
            }
        }
    }
    let mut result = 0;
    let mut already_check: HashSet<usize> = HashSet::new();

    for (k, v) in &friends_map {
        if already_check.contains(&k) {
            continue;
        }

        result += 1;
        already_check.insert(*k);
        insert_set(&mut already_check, &friends_map, &v);
    }
    result
}

fn insert_set(set: &mut HashSet<usize>, m: &HashMap<usize, Vec<usize>>, v: &Vec<usize>) {
    if v.iter().all(|x| set.contains(x)) {
        return;
    }
    v.iter().for_each(|x| {
        set.insert(*x);
    });

    v.iter().for_each(|x| {
        insert_set(set, m, m.get(x).unwrap());
    })
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
