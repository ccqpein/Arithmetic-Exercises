#![feature(exclusive_range_pattern)]
fn read_string(a: String) -> Vec<u8> {
    a.bytes().collect()
}

fn fill_data(a: &mut Vec<u8>) {
    //fill data to N * 512 bits + 448 bits
    //N * 64 * 8 + 56 * 8
    let length = a.len();
    let module = length % 64;
    match module {
        0..56 => {
            a.push(0b1000_0000u8);
            let left_bytes = 55 - module;
            for _ in 0..left_bytes {
                a.push(0b0000_0000u8)
            }
        }
        57..=63 => {
            a.push(0b1000_0000u8);
            let left_bytes = 63 - module + 56;
            for _ in 0..left_bytes {
                a.push(0b0000_0000u8)
            }
        }
        _ => {}
    }

    //then give length of this data
    let mut add_length = ((length * 8) as u64).to_be_bytes().to_vec();
    a.append(&mut add_length);
}

fn main() {
    // let mut a = vec![0b1011_0000u8];
    // fill_data(&mut a);
    // println!("{:?}", a.len());

    let mut b = [0b1001_1001u8; 67].to_vec();
    fill_data(&mut b);
    println!("{:?}", b);

    //println!("{:?}", (1 as u64).to_be_bytes());

    //let testcase0 = "test".to_string();
}
