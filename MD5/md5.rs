use std::io::prelude::*;
use std::io::BufReader;
use std::str;

fn read_n<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
where
    R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    // Do appropriate error handling for your situation
    let n = chunk.read_to_end(&mut buf).expect("Didn't read enough");
    assert_eq!(bytes_to_read as usize, n);
    buf
}

fn main() {
    let input_data = b"hello world,aaa";
    let mut reader = BufReader::new(&input_data[..]);

    let first = read_n(&mut reader, 5);
    let _ = read_n(&mut reader, 1);
    let second = read_n(&mut reader, 5);

    println!(
        "{:?}, {:?}",
        str::from_utf8(&first),
        str::from_utf8(&second),
    );
}
