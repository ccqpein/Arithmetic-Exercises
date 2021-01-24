pub fn longest_prefix(s: String) -> String {
    let mut indexes = vec![];
    let mut bb = s.bytes();
    let first = bb.next().unwrap();
    let mut ind = 0;

    for b in bb {
        ind += 1;
        if b == first {
            indexes.push(ind)
        }
    }

    longest_helper(&s, &indexes)
}

fn longest_helper(s: &str, l: &Vec<usize>) -> String {
    let len = s.len();
    for ind in l {
        if s[0..(len - ind)] == s[*ind..len] {
            return s[0..(len - ind)].to_string();
        }
    }
    return "".to_string();
}

pub fn longest_prefix2(s: String) -> String {
    let mut bb = s.bytes();
    let len = s.len();
    let first = bb.next().unwrap();
    let mut offset = 1;
    loop {
        if let Some(ind) = bb.position(|b| b == first) {
            println!("{:?}", ind + offset);
            if s[0..(len - (ind + offset))] == s[(ind + offset)..len] {
                return s[0..(len - (ind + offset))].to_string();
            }
            offset += ind + 1
        } else {
            break;
        }
    }
    "".to_string()
}

fn main() {
    dbg!(longest_prefix("level".to_string()));
    //dbg!(longest_helper("level", &vec![4]));

    dbg!(longest_prefix("ababab".to_string()));
    dbg!(longest_prefix("leetcodeleet".to_string()));
    dbg!(longest_prefix("a".to_string()));
}
