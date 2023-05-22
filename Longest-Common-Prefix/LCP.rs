pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return String::new();
    }

    let mut last = strs[0].clone().into_bytes();

    for i in 1..strs.len() {
        //dbg!(&last);
        last = helper(&last, &strs[i].as_bytes());
        if last.is_empty() {
            return String::new();
        }
    }
    String::from_utf8(last).unwrap()
}

fn helper(a: &[u8], b: &[u8]) -> Vec<u8> {
    //dbg!(String::from_utf8(a.clone().to_vec()).unwrap());
    //dbg!(String::from_utf8(b.clone().to_vec()).unwrap());

    let boundray = usize::min(a.len(), b.len());

    for i in 0..boundray {
        //dbg!(a[i]);
        if a[i] != b[i] {
            return a[0..i].to_vec();
        }
    }

    a[0..boundray].to_vec()
}

fn main() {
    assert_eq!(
        longest_common_prefix(
            vec!["flower", "flow", "flight"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        "fl".to_string()
    );

    assert_eq!(
        longest_common_prefix(
            vec!["dog", "racecar", "car"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        "".to_string()
    );

    assert_eq!(
        longest_common_prefix(vec![].into_iter().map(|s: &str| s.to_string()).collect()),
        "".to_string()
    );
}
