use std::collections::HashMap;

pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let xx = mat.len() as i32;
    let yy = mat.get(0).unwrap_or(&vec![]).len() as i32;
    let mut result = vec![vec![0; yy as usize]; xx as usize];
    dbg!(xx);
    dbg!(yy);
    for i in 0..xx as i32 {
        for j in 0..yy as i32 {
            result[i as usize][j as usize] = around(i, j, xx, yy, k)
                .iter()
                .map(|(x, y)| mat[*x as usize][*y as usize])
                .sum()
        }
    }
    result
}

fn around(x: i32, y: i32, xx: i32, yy: i32, k: i32) -> Vec<(i32, i32)> {
    let mut result = vec![];
    for c in x - k..=x + k {
        if c >= xx || c < 0 {
            continue;
        }

        for r in y - k..=y + k {
            if r >= yy || r < 0 {
                continue;
            }
            result.push((c, r))
        }
    }
    result
}

fn main() {
    dbg!(matrix_block_sum(
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        1
    ));
}
