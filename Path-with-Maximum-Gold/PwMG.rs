use std::collections::HashSet;

pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
    let mut largest = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] != 0 {
                let mut set = HashSet::new();
                set.insert((x as i32, y as i32));
                let ss = helper((x, y), &grid, &mut set);
                if ss > largest {
                    //println!("largest is {} at ({},{})", ss, x, y);
                    largest = ss
                }
            }
        }
    }
    largest
}

fn helper(
    this: (usize, usize),
    grid: &Vec<Vec<i32>>,
    set: &mut HashSet<(i32, i32)>,
    //record: i32,
) -> i32 {
    let mut largest = grid[this.0][this.1];
    for (a, b) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let xx = (this.0 as i32 + a, this.1 as i32 + b);
        if xx.0 < 0 || xx.0 >= grid.len() as i32 || xx.1 < 0 || xx.1 >= grid[0].len() as i32 {
            continue;
        }
        if !set.contains(&xx) && grid[xx.0 as usize][xx.1 as usize] != 0 {
            set.insert((xx.0, xx.1));
            let ss = grid[this.0][this.1] + helper((xx.0 as usize, xx.1 as usize), grid, set);
            set.remove(&(xx.0, xx.1));
            if ss > largest {
                largest = ss
            }
        }
    }
    //println!("largest is {} at ({},{})", largest, this.0, this.1);
    largest
}

fn main() {
    // let mut set = HashSet::new();
    // set.insert((2 as i32, 3 as i32));
    // set.insert((2 as i32, 2 as i32));
    // set.insert((0 as i32, 0 as i32));
    // let ss = helper(
    //     (2, 2),
    //     &vec![
    //         vec![1, 0, 7, 0, 0, 0],
    //         vec![2, 0, 6, 0, 1, 0],
    //         vec![3, 5, 6, 7, 4, 2],
    //         vec![4, 3, 1, 0, 2, 0],
    //         vec![3, 0, 5, 0, 20, 0],
    //     ],
    //     &mut set,
    // );
    // dbg!(ss);

    dbg!(get_maximum_gold(vec![
        vec![1, 0, 7],
        vec![2, 0, 6],
        vec![3, 4, 5],
        vec![0, 3, 0],
        vec![9, 0, 20]
    ]));

    dbg!(get_maximum_gold(vec![
        vec![0, 6, 0],
        vec![5, 8, 7],
        vec![0, 9, 0],
    ]));

    dbg!(get_maximum_gold(vec![
        vec![1, 0, 7, 0, 0, 0],
        vec![2, 0, 6, 0, 1, 0],
        vec![3, 5, 6, 7, 4, 2],
        vec![4, 3, 1, 0, 2, 0],
        vec![3, 0, 5, 0, 20, 0]
    ]));
}
