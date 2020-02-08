pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    let row = matrix.len();
    let col = matrix[0].len();

    let mut all_matrix: Vec<i32> = vec![];
    for si in 1..=col {
        if si > row {
            break;
        }
    }
}

fn main() {}
