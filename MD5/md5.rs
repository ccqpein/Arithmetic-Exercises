fn read_string(a: String) -> Vec<u8> {
    a.bytes().collect()
}

fn fill_data(a: &mut Vec<u8>) {
    //fill data to N * 512 bits + 448 bits
    //N * 64 * 8 + 56 * 8
    let length = a.len();
    if (length % 8)
}

fn main() {
    let testcase0 = "test".to_string();
}
