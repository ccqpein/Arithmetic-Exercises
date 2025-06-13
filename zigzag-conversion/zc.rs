// pub fn convert(s: String, num_rows: i32) -> String {
//     let mut jmp = 2 * (num_rows - 1) as usize;
//     if jmp == 0 {
//         jmp = 1
//     }
//     let s = s.bytes().collect::<Vec<_>>();
//     let mut res = vec![];
//     for i in 0..num_rows {
//         let mut ii = i as usize;
//         loop {
//             res.push(s[ii]);
//             ii += jmp;
//             if ii >= s.len() {
//                 break;
//             }
//         }
//     }
//     String::from_utf8(res).unwrap()
// }

pub fn convert(s: String, num_rows: i32) -> String {
    let mut jmp = 2 * (num_rows - 1) as usize;
    if jmp == 0 {
        jmp = 1
    }
    let s = s.bytes().collect::<Vec<_>>();
    let mut res = vec![];
    for i in 0..num_rows {
        let mut ii = i as usize;
        loop {
            res.push(s[ii]);
            ii += jmp;
            if ii >= s.len() {
                break;
            }
        }
    }
    String::from_utf8(res).unwrap()
}

fn main() {
    dbg!(convert("PAYPALISHIRING".to_string(), 3));
    dbg!(convert("AB".to_string(), 1));
}
