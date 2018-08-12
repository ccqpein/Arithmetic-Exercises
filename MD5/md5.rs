use std::io::prelude::*;
use std::io::BufReader;
use std::str;

//1 group = 512 bits = 16 * 32 bits
//512 bits = 64 bytes = 16 * 4 bytes
//448 bits = 56 bytes
//32 bits = 4 bytes
//MD5 give Hex chars out
fn read_n<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
where
    R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    // Do appropriate error handling for your situation
    let n = chunk.read_to_end(&mut buf);

    let size = bytes_to_read as usize;
    //if u < size {
    //
    //}

    //assert_eq!(bytes_to_read as usize, n);
    buf
}

fn read_group<R>(reader: R) -> Vec<u8>
where
    R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(64);
    // Do appropriate error handling for your situation
    let n = chunk.read_to_end(&mut buf).expect("Read issue error:");

    if n < 64 {
        if n < 56 {
            buf.push(0x10u8);
            for _ in 0..(56 - n + 1) {
                buf.push(0x00u8)
            }
        }
    }
    buf
}

fn main() {
    //let input_data = b"hello world,aaa";
    //let mut reader = BufReader::new(&input_data[..]);

    //let first = read_n(&mut reader, 5);
    //let _ = read_n(&mut reader, 1);
    //let second = read_n(&mut reader, 100);

    //println!(
    //    "{:?}, {:?}",
    //    str::from_utf8(&first),
    //    str::from_utf8(&second),
    //);

    let input_data = b"123456789A123456789B123456789C123456789D123456789E";
    let mut reader = BufReader::new(&input_data[..]);

    let word = read_group(&mut reader);
    println!("{:?}", str::from_utf8(&word));
}
