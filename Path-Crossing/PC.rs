pub fn is_path_crossing(path: String) -> bool {
    use std::collections::HashSet;
    let mut ss = HashSet::new();
    let mut now = (0, 0);
    ss.insert(now);
    for c in path.chars() {
        match c {
            'N' => {
                now.1 += 1;
                if let Some(_) = ss.get(&now) {
                    return true;
                } else {
                    ss.insert(now);
                }
            }
            'S' => {
                now.1 -= 1;
                if let Some(_) = ss.get(&now) {
                    return true;
                } else {
                    ss.insert(now);
                }
            }
            'E' => {
                now.0 += 1;
                if let Some(_) = ss.get(&now) {
                    return true;
                } else {
                    ss.insert(now);
                }
            }
            'W' => {
                now.0 -= 1;
                if let Some(_) = ss.get(&now) {
                    return true;
                } else {
                    ss.insert(now);
                }
            }
            _ => panic!(),
        }
    }
    false
}

fn main() {
    assert!(is_path_crossing("NESWW".to_string()));
    assert!(!is_path_crossing("NES".to_string()))
}
