pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
    let aa = board
        .iter()
        .map(|l| l.chars())
        .flatten()
        .collect::<Vec<_>>();

    let x_count = aa.iter().filter(|c| **c == 'X').count() as i32;
    let o_count = aa.iter().filter(|c| **c == 'O').count() as i32;

    if x_count != o_count && x_count - o_count != 1 {
        return false;
    }

    if x_count - o_count == 1 {
        if check_helper_O(&aa) {
            return false;
        }
    }

    if x_count == o_count {
        if check_helper_X(&aa) {
            return false;
        }
    }

    true
}

fn check_helper_O(board: &Vec<char>) -> bool {
    for cl in [
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 4, 8],
        [2, 4, 6],
    ] {
        match (board[cl[0]], board[cl[1]], board[cl[2]]) {
            ('O', 'O', 'O') => return true,
            _ => (),
        }
    }

    false
}

fn check_helper_X(board: &Vec<char>) -> bool {
    for cl in [
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 4, 8],
        [2, 4, 6],
    ] {
        match (board[cl[0]], board[cl[1]], board[cl[2]]) {
            ('X', 'X', 'X') => return true,
            _ => (),
        }
    }

    false
}

fn main() {
    assert!(!valid_tic_tac_toe(vec![
        "O  ".to_string(),
        "   ".to_string(),
        "   ".to_string()
    ]));

    assert!(!valid_tic_tac_toe(vec![
        "XOX".to_string(),
        " X ".to_string(),
        "   ".to_string()
    ]));

    assert!(valid_tic_tac_toe(vec![
        "XOX".to_string(),
        "O O".to_string(),
        "XOX".to_string()
    ]));

    assert!(!valid_tic_tac_toe(vec![
        "XXX".to_string(),
        "   ".to_string(),
        "OOO".to_string()
    ]));

    assert!(!valid_tic_tac_toe(vec![
        "XXX".to_string(),
        "XOO".to_string(),
        "OO ".to_string()
    ]));
}
