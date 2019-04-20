pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    let row = matrix.len();
    let col = matrix[0].len();

    if row < 2 || col < 2 {
        return true;
    }

    let compare_list = {
        let mut temp = vec![(0, 0)];
        for c in 1..col - 1 {
            temp.push((0, c))
        }
        for r in 1..row - 1 {
            temp.push((r, 0))
        }
        temp
    };

    //println!("{:?}", compare_list);

    fn match_each(m: &Vec<Vec<i32>>, ind: (usize, usize), row: usize, col: usize) -> bool {
        let first_v = m[ind.0][ind.1];
        let mut row_ = ind.0 + 1;
        let mut col_ = ind.1 + 1;
        while row_ < row && col_ < col {
            if first_v != m[row_][col_] {
                return false;
            } else {
                row_ += 1;
                col_ += 1;
            }
        }
        return true;
    }

    for ele in compare_list {
        if !match_each(&matrix, ele, row, col) {
            return false;
        }
    }
    return true;
}

fn main() {
    let test_0 = vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]];
    println!("{:?}", is_toeplitz_matrix(test_0)); //=> true

    let test_1 = vec![vec![1, 2], vec![2, 2]];
    println!("{:?}", is_toeplitz_matrix(test_1)); //=> false

    let test_2 = vec![vec![84]];
    println!("{:?}", is_toeplitz_matrix(test_2)); //=> true

    let test_3 = vec![vec![11, 74, 0, 93], vec![40, 11, 74, 7]];
    println!("{:?}", is_toeplitz_matrix(test_3)); //=> false
}
