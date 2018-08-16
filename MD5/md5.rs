use std::io::prelude::*;
use std::io::BufReader;
use std::str;

//1 group = 512 bits = 16 * 32 bits
//512 bits = 64 bytes = 16 * 4 bytes
//448 bits = 56 bytes
//32 bits = 4 bytes
//MD5 give Hex chars out
/*
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
            for _ in 0..(55 - n) {
                buf.push(0x00u8)
            }
        } else if n > 56 {
            buf.push(0x10u8);
            for _ in 0..(63 - n) {
                buf.push(0x00u8)
            }
            for _ in 0..56 {
                buf.push(0x00u8)
            }
        }
        return buf;
    }

    buf
}*/

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

    //length of orginal data tail 64 bits
    //let mut len_data = (origin_len * 8) % u64::max_value();
    //len_data & 0xff00000000000000

    //let mut bit_cache = 0xff00000000000000;
    //for i in 0..8 {
    //    buf.push(((len_data & bit_cache) >> (64 - i * 8)) as u8);
    //    len_data >>= (64 - i * 8)
    //}

    buf
}

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
