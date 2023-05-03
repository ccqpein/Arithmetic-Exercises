use std::collections::HashSet;

pub fn max_unique_split(s: String) -> i32 {
    let mut set = HashSet::new();
    //handle(&s, &mut set);
    handle2(&s, &mut set) as i32
    //dbg!(&set);
    //set.len() as i32
}

fn handle(s: &str, set: &mut HashSet<String>) -> bool {
    let len = s.len();
    if len == 0 {
        return true;
    }

    let mut i = 1;
    let mut a = String::new();
    loop {
        if i > len {
            return false;
        }

        a = s[0..i].to_string();
        if set.contains(&a) {
            i += 1;
        } else {
            set.insert(a.clone());
            if handle(&s[i..len], set) {
                return true;
            } else {
                set.remove(&a);
                i += 1;
            }
        }
    }
}

fn handle2(s: &str, set: &mut HashSet<String>) -> usize {
    //dbg!(s);
    if s.len() == 0 {
        return 0;
    }
    let mut max = 0;
    for i in 1..=s.len() {
        let a = s[0..i].to_string();
        if set.contains(&a) {
            continue;
        } else {
            set.insert(a.clone());
            //dbg!(a.clone());
            let vv = handle2(&s[i..s.len()], set);
            //dbg!(vv);
            if vv + 1 >= max {
                max = vv + 1;
            }
            set.remove(&a);
        }
    }
    max
}

fn main() {
    assert_eq!(5, max_unique_split("ababccc".to_string()));
    assert_eq!(2, max_unique_split("aba".to_string()));
    assert_eq!(1, max_unique_split("aa".to_string()));
    assert_eq!(11, max_unique_split("wwwzfvedwfvhsww".to_string()));
}
