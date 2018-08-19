use std::io::prelude::*;
use std::io::BufReader;
use std::str;

//1 group = 512 bits = 16 * 32 bits
//512 bits = 64 bytes = 16 * 4 bytes
//448 bits = 56 bytes
//32 bits = 4 bytes
//MD5 give Hex chars out

fn read_group<R>(reader: R) -> Vec<u8>
where
    R: Read,
{
    let mut buf = vec![];
    let bytes = reader.bytes();
    let mut time = 0;

    let mut origin_len = 0;
    for byte in bytes {
        time += 1;
        origin_len += 1;
        buf.push(byte.unwrap());
        if time == 64 {
            time = 0;
        }
    }

    if time < 64 {
        if time < 56 {
            buf.push(0x10u8);
            for _ in 0..(55 - time) {
                buf.push(0x00u8)
            }
        } else if time > 56 {
            buf.push(0x10u8);
            for _ in 0..(63 - time) {
                buf.push(0x00u8)
            }
            for _ in 0..56 {
                buf.push(0x00u8)
            }
        }
    }

    //println!("{:?}", last_64_bits(&origin_len));

    buf.append(&mut last_64_bits(&origin_len));

    buf
}

//Put length in and return last 64 bits
fn last_64_bits(length: &u64) -> Vec<u8> {
    let mut len_last64 = length % u64::max_value();
    let mut bit_cache = 0xff00000000000000;
    let mut buf: Vec<u8> = vec![];

    for i in 0..8 {
        buf.push(((len_last64 & bit_cache) >> (56 - i * 8)) as u8);
        bit_cache >>= 8;
    }

    return buf;
}

//A=0x67452301, B=0xefcdab89, C=0x98badcfe, D=0x10325476

fn main() {
    let input_data = b"123456789A123456789B123456789C123456789D123456789E123456789F";
    let mut reader = BufReader::new(&input_data[..]);

    let word = read_group(&mut reader);
    println!("{}", word.len());
    println!("{:?}", str::from_utf8(&word));

    println!("{}", 18446744073709551615 % u64::max_value());
    println!("{}", 184 % u64::max_value());
    println!("{}", 0xafu8 >> 2); //10101111 -> 00101011

    println!("{}", 12249790986447749120 & 0xff00000000000000 as u64);

    println!("{:?}", last_64_bits(&(0x1200130014001500 as u64)));
}
