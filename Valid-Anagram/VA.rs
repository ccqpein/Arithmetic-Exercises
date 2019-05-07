pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    };

    let length = s.len();
    let mut s_bytes = s.into_bytes();
    let mut t_bytes = t.into_bytes();

    s_bytes.sort();
    t_bytes.sort();
    println!("{:?},{:?}", s_bytes, t_bytes);

    for i in 0..length {
        if s_bytes[i] != t_bytes[i] {
            return false;
        }
    }

    true
}

fn main() {
    println!(
        "{:?}",
        is_anagram(String::from("anagram"), String::from("nagaram"))
    );
}
