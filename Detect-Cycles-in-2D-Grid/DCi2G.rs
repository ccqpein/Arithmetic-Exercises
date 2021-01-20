use std::collections::{HashMap, HashSet};

pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
    let mut table = HashMap::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let en = table.entry(&grid[r][c]).or_insert(vec![]);
            en.push((r, c))
        }
    }
    //dbg!(&table);
    let mn = (grid.len(), grid[0].len());

    for (start_char, ind_l) in table.iter() {
        let mut stack = HashSet::new();
        // if **start_char != 'c' {
        //     continue;
        // }
        for ind in ind_l {
            if !stack.contains(ind) {
                if dfs(&grid, start_char, &mn, ind.0, ind.1, &mut stack) {
                    //println!("start:{}, at: {:?}", start_char, ind);
                    return true;
                } else {
                    continue;
                }
            }
        }
    }
    false
}

fn dfs(
    grid: &Vec<Vec<char>>,
    start_char: &char,
    mn: &(usize, usize),
    r: usize,
    c: usize,
    stack: &mut HashSet<(usize, usize)>,
) -> bool {
    if stack.contains(&(r, c)) {
        //println!("{:?}", (r, c));
        return true;
    }

    if grid[r][c] != *start_char {
        return false;
    }

    stack.insert((r, c));

    //println!("this: {:?}", (r, c));
    let next: Vec<(usize, usize)> = {
        let rr = r as i32;
        let cc = c as i32;
        [(rr + 1, cc), (rr, cc - 1), (rr - 1, cc), (rr, cc + 1)]
            .iter()
            .cloned()
            .filter(|v| {
                v.0 >= 0
                    && v.0 < mn.0 as i32
                    && v.1 >= 0
                    && v.1 < mn.1 as i32
                    && !stack.contains(&(v.0 as usize, v.1 as usize))
            })
            .map(|(rr, cc)| (rr as usize, cc as usize))
            .collect()
    };

    for (rr, cc) in next.iter() {
        if dfs(grid, start_char, mn, *rr, *cc, stack) {
            return true;
        }
    }
    false
}

fn main() {
    let testcase0 = vec![
        vec!['a', 'a', 'a', 'a'],
        vec!['a', 'b', 'b', 'a'],
        vec!['a', 'b', 'b', 'a'],
        vec!['a', 'a', 'a', 'a'],
    ];

    assert!(contains_cycle(testcase0));

    let testcase1 = vec![
        vec!['c', 'c', 'c', 'a'],
        vec!['c', 'd', 'c', 'c'],
        vec!['c', 'c', 'e', 'c'],
        vec!['f', 'c', 'c', 'c'],
    ];

    assert!(contains_cycle(testcase1));

    let testcase2 = vec![
        vec!['d', 'b', 'b'],
        vec!['c', 'a', 'a'],
        vec!['b', 'a', 'c'],
        vec!['c', 'c', 'c'],
        vec!['d', 'd', 'a'],
    ];

    assert!(!contains_cycle(testcase2));
}
