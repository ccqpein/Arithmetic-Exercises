// pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
//     use std::collections::HashSet;
//     let all_bs: HashSet<(_, _)> = buildings.into_iter().map(|b| (b[0], b[1])).collect();
//     let mut cache0 = HashSet::new();
//     let mut cache1 = HashSet::new();
//     let mut cache2 = HashSet::new();
//     let mut cache3 = HashSet::new();
//     let mut all_edges = HashSet::new();
//     for l in 1..=n {
//         for c in 1..=n {
//             if !cache0.contains(&c) && all_bs.contains(&(c, l)) {
//                 cache0.insert(c);
//                 all_edges.insert((c, l));
//             }
//         }
//     }

//     for l in (1..=n).rev() {
//         for c in 1..=n {
//             if !cache1.contains(&c) && all_bs.contains(&(c, l)) {
//                 cache1.insert(c);
//                 all_edges.insert((c, l));
//             }
//         }
//     }

//     for c in 1..=n {
//         for l in 1..=n {
//             if !cache2.contains(&l) && all_bs.contains(&(c, l)) {
//                 cache2.insert(l);
//                 all_edges.insert((c, l));
//             }
//         }
//     }

//     for c in (1..=n).rev() {
//         for l in 1..=n {
//             if !cache3.contains(&l) && all_bs.contains(&(c, l)) {
//                 cache3.insert(l);
//                 all_edges.insert((c, l));
//             }
//         }
//     }
//     dbg!(&all_edges);
//     (all_bs.len() - all_edges.len()) as i32
// }

pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    // let mut x: HashMap<i32, [(i32, i32); 2]> =
    //     (1..=n).into_iter().map(|c| (c, [(c, 0), (c, n)])).collect();
    // let mut y: HashMap<i32, [(i32, i32); 2]> =
    //     (1..=n).into_iter().map(|l| (l, [(0, l), (n, l)])).collect();
    let mut x = HashMap::new();
    let mut y = HashMap::new();

    for b in &buildings {
        let this_x = b[0];
        let this_y = b[1];

        let x_en = x.entry(this_x).or_insert((n, 0));
        *x_en = (b[1].min(x_en.0), b[1].max(x_en.1));

        let y_en = y.entry(this_y).or_insert((n, 0));
        *y_en = (b[0].min(y_en.0), b[0].max(y_en.1));
    }
    //dbg!(&x);
    //dbg!(&y);
    buildings.len() as i32
        - buildings
            .into_iter()
            .map(|b| {
                let yy = x[&b[0]];
                let xx = y[&b[1]];
                //dbg!(&b);
                if b[1] != yy.0 && b[1] != yy.1 && b[0] != xx.0 && b[0] != xx.1 {
                    0
                } else {
                    1
                }
            })
            .sum::<i32>()
}

fn main() {
    dbg!(count_covered_buildings(
        3,
        vec![
            vec![1, 2],
            vec![2, 1],
            vec![3, 1],
            vec![1, 1],
            vec![3, 3],
            vec![3, 2]
        ]
    ));
}
