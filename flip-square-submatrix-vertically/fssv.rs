pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
    let aa = flip_helper(
        grid.get(x as usize..(x + k) as usize).unwrap().to_vec(),
        y as usize,
        k as usize,
    );

    for (ind, a) in aa.into_iter().enumerate() {
        grid[ind + x as usize] = a;
    }

    grid
}

fn flip_helper(mut grid: Vec<Vec<i32>>, y: usize, k: usize) -> Vec<Vec<i32>> {
    let mut ind = 0;
    for l in grid.clone().into_iter().rev() {
        let a = grid.get_mut(ind).unwrap();
        for (ii, v) in l[y..y + k].iter().enumerate() {
            a[ii + y] = *v
        }
        ind += 1;
    }
    grid
}

fn main() {
    dbg!(flip_helper(
        vec![vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]],
        0,
        3,
    ));

    dbg!(flip_helper(vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2],], 2, 2,));

    dbg!(reverse_submatrix(
        vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16]
        ],
        1,
        0,
        3
    ));

    dbg!(reverse_submatrix(
        vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2],],
        0,
        2,
        2
    ));

    dbg!(reverse_submatrix(
        vec![
            vec![4, 20, 8, 20],
            vec![2, 16, 3, 12],
            vec![3, 12, 17, 1],
            vec![3, 13, 2, 13]
        ],
        1,
        1,
        1
    ));
}
