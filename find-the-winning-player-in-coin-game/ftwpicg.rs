pub fn losing_player(x: i32, y: i32) -> String {
    if y / 4 >= x {
        if x % 2 == 0 {
            "Bob".to_string()
        } else {
            "Alice".to_string()
        }
    } else {
        if (y / 4) % 2 == 0 {
            "Bob".to_string()
        } else {
            "Alice".to_string()
        }
    }
}

fn main() {
    assert_eq!("Alice".to_string(), losing_player(2, 7));
    assert_eq!("Bob".to_string(), losing_player(4, 11));
    assert_eq!("Alice".to_string(), losing_player(1, 8));
}
