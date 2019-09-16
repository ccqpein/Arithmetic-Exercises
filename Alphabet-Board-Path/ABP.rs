pub fn alphabet_board_path(target: String) -> String {
    use std::collections::HashMap;

    let mut board_map = HashMap::new();
    let board = ["abcde", "fghij", "klmno", "pqrst", "uvwxy", "z"];

    for r in 0..board.len() {
        let inner_bytes = board[r].as_bytes();
        for c in 0..board[r].len() {
            board_map.insert(inner_bytes[c], (r as i32, c as i32));
        }
    }
    //dbg!(board_map);

    let mut now = (0, 0);
    let mut result = vec![];
    for c in target.as_str().bytes() {
        let tg = board_map.get(&c).unwrap();
        let (row_diff, col_diff) = (tg.0 - now.0, tg.1 - now.1);
        match ({ row_diff >= 0 }, { col_diff >= 0 }) {
            (true, true) => {
                for _ in 0..row_diff {
                    result.push(b'D')
                }
                for _ in 0..col_diff {
                    result.push(b'R')
                }
            } //I want to use repeat, but it is nightly only
            (true, false) => {
                for _ in 0..row_diff {
                    result.push(b'D')
                }
                for _ in 0..col_diff.abs() {
                    result.push(b'L')
                }
            }
            (false, true) => {
                for _ in 0..row_diff.abs() {
                    result.push(b'U')
                }
                for _ in 0..col_diff {
                    result.push(b'R')
                }
            }
            (false, false) => {
                for _ in 0..row_diff.abs() {
                    result.push(b'U')
                }
                for _ in 0..col_diff.abs() {
                    result.push(b'L')
                }
            }
        }
        result.push(b'!');
        now = tg.clone();
    }

    String::from_utf8(result).unwrap()
}

fn main() {
    assert_eq!(
        alphabet_board_path("leet".to_string()),
        "DDR!UURRR!!DDD!".to_string()
    );
    assert_eq!(
        alphabet_board_path("code".to_string()),
        "RR!DDRR!UUL!R!".to_string()
    );
}
