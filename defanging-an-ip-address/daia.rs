pub fn defang_i_paddr(address: String) -> String {
    String::from_utf8(
        address
            .bytes()
            .map(|b| match b {
                b'.' => vec![b'[', b'.', b']'],
                _ => vec![b],
            })
            .flatten()
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn main() {
    assert_eq!(
        defang_i_paddr("1.1.1.1".to_string()),
        "1[.]1[.]1[.]1".to_string()
    );

    assert_eq!(
        defang_i_paddr("255.100.50.0".to_string()),
        "255[.]100[.]50[.]0".to_string()
    );
}
