pub fn can_be_equal(s1: String, s2: String) -> bool {
    use std::collections::HashSet;
    let s1 = s1.into_bytes();
    let s2 = s2.into_bytes();

    let mut set0 = HashSet::new();
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    let mut set3 = HashSet::new();

    set0.insert(s1[0]);
    set0.insert(s1[2]);
    set1.insert(s2[0]);
    set1.insert(s2[2]);

    set2.insert(s1[1]);
    set2.insert(s1[3]);
    set3.insert(s2[1]);
    set3.insert(s2[3]);

    set0.difference(&set1).count() == 0 && set2.difference(&set3).count() == 0
}

fn main() {}
