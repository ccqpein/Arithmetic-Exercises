pub fn judge_circle(moves: String) -> bool {
    let mut h = 0;
    let mut c = 0;
    for cc in moves.chars() {
        match cc {
            'U' => {
                h += 1;
            }
            'D' => {
                h -= 1;
            }
            'L' => {
                c -= 1;
            }
            'R' => c += 1,
            _ => unreachable!(),
        }
    }
    h == 0 && c == 0
}

fn main() {
    assert!(judge_circle("UD".to_string()));
    assert!(!judge_circle("LL".to_string()));
}
