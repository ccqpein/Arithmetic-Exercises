pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    use std::cmp::*;
    let row = matrix.len();
    let col = matrix[0].len();

    let whole_size = min(row, col);

    let mut count = 0;
    for r in 0..row {
        for c in 0..col {
            for si in 1..=whole_size {
                if r + si > row || c + si > col {
                    break;
                }

                if pick_matrix(&matrix, r, c, si) {
                    count += 1;
                } else {
                    continue;
                }
            }
        }
    }
    count
}

fn pick_matrix(matrix: &Vec<Vec<i32>>, row: usize, col: usize, size: usize) -> bool {
    for r in row..row + size {
        if !matrix[r][col..col + size].iter().all(|x| *x == 1) {
            return false;
        }
    }

    return true;
}

fn main() {
    dbg!(pick_matrix(
        &vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]],
        0,
        1,
        2
    ));

    dbg!(count_squares(vec![
        vec![0, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![0, 1, 1, 1]
    ]));
}
