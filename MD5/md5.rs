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

// fn function_F(x: &[u8; 4], y: &[u8; 4], z: &[u8; 4]) -> [u8; 4] {
//     let mut temp0: [u8; 4] = [0, 0, 0, 0];
//     for i in 0..4 {
//         temp0[i] = x[i] & y[i];
//     }

//     let mut temp1: [u8; 4] = [0, 0, 0, 0];
//     for i in 0..4 {
//         temp1[i] = !x[i] & z[i];
//     }

//     let mut temp2: [u8; 4] = [0, 0, 0, 0];
//     for i in 0..4 {
//         temp2[i] = temp0[i] | temp1[i];
//     }

//     temp2
// }

fn function_F(x: &u32, y: &u32, z: &u32) -> u32 {
    let mut temp0: u32 = 0;
    temp0 = x & y;

    let mut temp1: u32 = 0;
    temp1 = !x & z;

    let mut temp2: u32 = 0;
    temp2 = temp0 | temp1;

    temp2
}

fn function_G(x: &u32, y: &u32, z: &u32) -> u32 {
    let mut temp0: u32 = 0;
    temp0 = x & z;

    let mut temp1: u32 = 0;
    temp1 = y & !z;

    let mut temp2: u32 = 0;
    temp2 = temp0 | temp1;

    temp2
}

fn function_H(x: &u32, y: &u32, z: &u32) -> u32 {
    let mut temp0: u32 = 0;
    temp0 = x ^ y ^ z;

    temp0
}

fn function_I(x: &u32, y: &u32, z: &u32) -> u32 {
    let mut temp0: u32 = 0;
    temp0 = y ^ (x | !z);

    temp0
}

fn bits_plus(x: &u32, y: &u32) -> u32 {
    //use std::mem;
    use std::num::Wrapping;
    //let mut result: u32 = 0;
    //let x_inner = Wrapping(unsafe { mem::transmute::<[u8; 4], u32>(*x) });
    //let y_inner = Wrapping(unsafe { mem::transmute::<[u8; 4], u32>(*y) });

    //(*x as u64 + *y as u64) as u32
    x.wrapping_add(*y)

    //result = unsafe { mem::transmute::<u32, [u8; 4]>(re.0) };
}

fn offset_func(x: u32, offset: u32) -> u32 {
    //use std::mem;
    //let x_inner: u32 = unsafe { mem::transmute::<[u8; 4], u32>(x) };
    x.rotate_left(offset)

    //x.to_be_bytes()
}

fn wrap_function<FF>(a: &mut u32, b: &u32, c: &u32, d: &u32, mj: &u32, s: &u32, ti: &u32, fu: FF)
where
    FF: Fn(&u32, &u32, &u32) -> u32,
{
    *a = bits_plus(
        b,
        &offset_func(
            *&bits_plus(a, &(bits_plus(&bits_plus(&fu(b, c, d), mj), ti))),
            *s,
        ),
    )
}

fn cut_to_bytes(v: Vec<u8>) -> [u32; 16] {
    use std::mem;
    let mut result: [u32; 16] = [0; 16];
    let mut v = v.iter();
    for i in 0..16 {
        let mut aa: [u8; 4] = [0; 4];
        for ii in 0..4 {
            aa[ii] = *v.next().unwrap();
        }
        result[i] = unsafe { mem::transmute::<[u8; 4], u32>(aa) };
    }
    result
}

fn loop_operation(group: Vec<u8>) -> [u32; 4] {
    use std::mem;
    let mut A = 0x01234567_u32;
    let mut B = 0x89abcdef_u32;
    let mut C = 0xfedcba98_u32;
    let mut D = 0x76543210_u32;

    let mut a = A.clone();
    let mut b = B.clone();
    let mut c = C.clone();
    let mut d = D.clone();

    let mut group = group;
    while group.len() > 0 {
        let left: Vec<_> = group.drain(64..).collect();
        let group_slice: [u32; 16] = cut_to_bytes(group);

        {
            //round 1
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[0],
                &7_u32,
                &0xd76aa478_u32,
                function_F,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[1],
                &12_u32,
                &0xe8c7b756_u32,
                function_F,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[2],
                &17_u32,
                &0x242070db_u32,
                function_F,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[3],
                &22_u32,
                &0xc1bdceee_u32,
                function_F,
            );
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[4],
                &7_u32,
                &0xf57c0faf_u32,
                function_F,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[5],
                &12_u32,
                &0x4787c62a_u32,
                function_F,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[6],
                &17_u32,
                &0xa8304613_u32,
                function_F,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[7],
                &22_u32,
                &0xfd469501_u32,
                function_F,
            );
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[8],
                &7_u32,
                &0x698098d8_u32,
                function_F,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[9],
                &12_u32,
                &0x8b44f7af_u32,
                function_F,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[10],
                &17_u32,
                &0xffff5bb1_u32,
                function_F,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[11],
                &22_u32,
                &0x895cd7be_u32,
                function_F,
            );
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[12],
                &7_u32,
                &0x6b901122_u32,
                function_F,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[13],
                &12_u32,
                &0xfd987193_u32,
                function_F,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[14],
                &17_u32,
                &0xa679438e_u32,
                function_F,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[15],
                &22_u32,
                &0x49b40821_u32,
                function_F,
            );

            //round 2
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[1],
                &5_u32,
                &0xf61e2562_u32,
                function_G,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[6],
                &9_u32,
                &0xc040b340_u32,
                function_G,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[11],
                &14_u32,
                &0x265e5a51_u32,
                function_G,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[0],
                &20_u32,
                &0xe9b6c7aa_u32,
                function_G,
            );
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[5],
                &5_u32,
                &0xd62f105d_u32,
                function_G,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[10],
                &9_u32,
                &0x02441453_u32,
                function_G,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[15],
                &14_u32,
                &0xd8a1e681_u32,
                function_G,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[4],
                &20_u32,
                &0xe7d3fbc8_u32,
                function_G,
            );
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[9],
                &5_u32,
                &0x21e1cde6_u32,
                function_G,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[14],
                &9_u32,
                &0xc33707d6_u32,
                function_G,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[3],
                &14_u32,
                &0xf4d50d87_u32,
                function_G,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[8],
                &20_u32,
                &0x455a14ed_u32,
                function_G,
            );
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[13],
                &5_u32,
                &0xa9e3e905_u32,
                function_G,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[2],
                &9_u32,
                &0xfcefa3f8_u32,
                function_G,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[7],
                &14_u32,
                &0x676f02d9_u32,
                function_G,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[12],
                &20_u32,
                &0x8d2a4c8a_u32,
                function_G,
            );

            //round 3
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[5],
                &4_u32,
                &0xfffa3942_u32,
                function_H,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[8],
                &11_u32,
                &0x8771f681_u32,
                function_H,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[11],
                &16_u32,
                &0x6d9d6122_u32,
                function_H,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[14],
                &23_u32,
                &0xfde5380c_u32,
                function_H,
            );
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[1],
                &4_u32,
                &0xa4beea44_u32,
                function_H,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[4],
                &11_u32,
                &0x4bdecfa9_u32,
                function_H,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[7],
                &16_u32,
                &0xf6bb4b60_u32,
                function_H,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[10],
                &23_u32,
                &0xbebfbc70_u32,
                function_H,
            );
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[13],
                &4_u32,
                &0x289b7ec6_u32,
                function_H,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[0],
                &11_u32,
                &0xeaa127fa_u32,
                function_H,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[3],
                &16_u32,
                &0xd4ef3085_u32,
                function_H,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[6],
                &23_u32,
                &0x04881d05_u32,
                function_H,
            );
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[9],
                &4_u32,
                &0xd9d4d039_u32,
                function_H,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[12],
                &11_u32,
                &0xe6db99e5_u32,
                function_H,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[15],
                &16_u32,
                &0x1fa27cf8_u32,
                function_H,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[2],
                &23_u32,
                &0xc4ac5665_u32,
                function_H,
            );

            //round 4
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[0],
                &6_u32,
                &0xf4292244_u32,
                function_I,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[7],
                &10_u32,
                &0x432aff97_u32,
                function_I,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[14],
                &15_u32,
                &0xab9423a7_u32,
                function_I,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[5],
                &21_u32,
                &0xfc93a039_u32,
                function_I,
            );
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[12],
                &6_u32,
                &0x655b59c3_u32,
                function_I,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[3],
                &10_u32,
                &0x8f0ccc92_u32,
                function_I,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[10],
                &15_u32,
                &0xffeff47d_u32,
                function_I,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[1],
                &21_u32,
                &0x85845dd1_u32,
                function_I,
            );
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[8],
                &6_u32,
                &0x6fa87e4f_u32,
                function_I,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[15],
                &10_u32,
                &0xfe2ce6e0_u32,
                function_I,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[6],
                &15_u32,
                &0xa3014314_u32,
                function_I,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[13],
                &21_u32,
                &0x4e0811a1_u32,
                function_I,
            );
            wrap_function(
                &mut a,
                &b,
                &c,
                &d,
                &group_slice[4],
                &6_u32,
                &0xf7537e82_u32,
                function_I,
            );
            wrap_function(
                &mut d,
                &a,
                &b,
                &c,
                &group_slice[11],
                &10_u32,
                &0xbd3af235_u32,
                function_I,
            );
            wrap_function(
                &mut c,
                &d,
                &a,
                &b,
                &group_slice[2],
                &15_u32,
                &0x2ad7d2bb_u32,
                function_I,
            );
            wrap_function(
                &mut b,
                &c,
                &d,
                &a,
                &group_slice[9],
                &21_u32,
                &0xeb86d391_u32,
                function_I,
            );
        }

        A = bits_plus(&a, &A);
        B = bits_plus(&b, &B);
        C = bits_plus(&c, &C);
        D = bits_plus(&d, &D);

        a = A.clone();
        b = B.clone();
        c = C.clone();
        d = D.clone();

        group = left;
    }

    [
        // unsafe { mem::transmute::<[u8; 4], u32>(a) },
        // unsafe { mem::transmute::<[u8; 4], u32>(b) },
        // unsafe { mem::transmute::<[u8; 4], u32>(c) },
        // unsafe { mem::transmute::<[u8; 4], u32>(d) },
        a, b, c, d,
    ]
}

fn main() {
    // let mut a = vec![0b1011_0000u8];
    // fill_data(&mut a);
    // println!("{:?}", a.len());

    // let mut b = [0b1001_1001u8; 67].to_vec();
    // fill_data(&mut b);
    // println!("{:?}", b);

    //println!("{:?}", (1 as u64).to_be_bytes());

    //let testcase0 = "test".to_string();

    // let x: [u8; 4] = [0b1011_0000u8, 0b1011_0000u8, 0b1011_0000u8, 0b1011_0000u8];
    // let y: [u8; 4] = [0b0011_1010u8, 0b0011_1010u8, 0b0011_1010u8, 0b0011_1010u8];
    // let z: [u8; 4] = [0b1101_1100u8, 0b1101_1100u8, 0b1101_1100u8, 0b1101_1100u8];

    let mut test0 = read_string(String::from("a"));
    fill_data(&mut test0);
    //println!("{:?}", loop_operation(test0));
    loop_operation(test0)
        .iter()
        .map(|x| println!("{:X}", x))
        .collect::<()>(); //Wrong answer, need use u32
}
