pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut iter_s = s.chars();
    let mut last_char = iter_s.next().unwrap();
    let mut this_ind = 1;
    let mut start = 0;
    for b in iter_s {
        if b != last_char && (this_ind - start) > 2 {
            result.push(vec![start, this_ind - 1]);
            start = this_ind;
        } else if b != last_char {
            start = this_ind;
        }
        last_char = b;
        this_ind += 1;
    }

    if (this_ind - start) > 2 {
        result.push(vec![start, this_ind - 1]);
    }

    result
}

fn main() {
    dbg!(large_group_positions("abbxxxxzzy".to_string()));
    dbg!(large_group_positions("abcdddeeeeaabbbcd".to_string()));
    dbg!(large_group_positions("aaa".to_string()));
    dbg!(large_group_positions("bababbabaa".to_string()));
}
