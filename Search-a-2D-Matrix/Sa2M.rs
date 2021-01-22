pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut lines = matrix.iter();
    while let Some(l) = lines.next() {
        if l[0] > target {
            return false;
        } else if let Some(_) = l.iter().find(|&v| *v == target) {
            return true;
        }
    }
    false
}

fn main() {
    let test0 = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert!(search_matrix(test0, 3));

    let test1 = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert_eq!(search_matrix(test1, 13), false);
}
