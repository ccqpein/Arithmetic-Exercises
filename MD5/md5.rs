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

fn function_F(x: &[u8; 4], y: &[u8; 4], z: &[u8; 4]) -> [u8; 4] {
    let mut temp0: [u8; 4] = [0, 0, 0, 0];
    for i in 0..4 {
        temp0[i] = x[i] & y[i];
    }
    dbg!(temp0);
    dbg!(x);

    let mut temp1: [u8; 4] = [0, 0, 0, 0];
    for i in 0..4 {
        temp1[i] = !x[i] & z[i];
    }
    dbg!(temp1);

    let mut temp2: [u8; 4] = [0, 0, 0, 0];
    for i in 0..4 {
        temp2[i] = temp0[i] | temp1[i];
    }

    temp2
}

fn function_G(x: &[u8; 4], y: &[u8; 4], z: &[u8; 4]) -> [u8; 4] {
    let mut temp0: [u8; 4] = [0, 0, 0, 0];
    for i in 0..4 {
        temp0[i] = x[i] & z[i];
    }
    dbg!(temp0);
    dbg!(x);

    let mut temp1: [u8; 4] = [0, 0, 0, 0];
    for i in 0..4 {
        temp1[i] = y[i] & !z[i];
    }
    dbg!(temp1);

    let mut temp2: [u8; 4] = [0, 0, 0, 0];
    for i in 0..4 {
        temp2[i] = temp0[i] | temp1[i];
    }

    temp2
}

fn function_H(x: &[u8; 4], y: &[u8; 4], z: &[u8; 4]) -> [u8; 4] {
    let mut temp0: [u8; 4] = [0, 0, 0, 0];
    for i in 0..4 {
        temp0[i] = x[i] ^ y[i] ^ z[i];
    }
    temp0
}

fn function_I(x: &[u8; 4], y: &[u8; 4], z: &[u8; 4]) -> [u8; 4] {
    let mut temp0: [u8; 4] = [0, 0, 0, 0];
    for i in 0..4 {
        temp0[i] = y[i] ^ (x[i] | !z[i]);
    }
    temp0
}

fn bits_plus(x: &[u8; 4], y: &[u8; 4]) -> [u8; 4] {
    let mut result: [u8; 4] = [0, 0, 0, 0];

    for i in 0..4 {
        result[i] = x[i] + y[i]
    }

    result
}

//:= TODO
//fn offset_func()

//:= todo
fn wrap_function<FF>(
    a: &mut [u8; 4],
    b: &[u8; 4],
    c: &[u8; 4],
    d: &[u8; 4],
    mj: &[u8; 4],
    s: &[u8; 4],
    ti: &[u8; 4],
    fu: FF,
) where
    FF: Fn(&[u8; 4], &[u8; 4], &[u8; 4]) -> [u8; 4],
{
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

    let x: [u8; 4] = [0b1011_0000u8, 0b1011_0000u8, 0b1011_0000u8, 0b1011_0000u8];
    let y: [u8; 4] = [0b0011_1010u8, 0b0011_1010u8, 0b0011_1010u8, 0b0011_1010u8];
    let z: [u8; 4] = [0b1101_1100u8, 0b1101_1100u8, 0b1101_1100u8, 0b1101_1100u8];
    dbg!(function_I(&x, &y, &z));

    dbg!(bits_plus(&x, &y));
}
