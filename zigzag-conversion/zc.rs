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
    let s = s.bytes();
    let mut res = vec![];

    (0..num_rows).for_each(|_| res.push(vec![]));

    let mut r = 0;
    let mut f = true;
    for b in s {
        res[r].push(b);
        if num_rows == 1 {
            continue;
        }
        if r == (num_rows as usize - 1) && f {
            f = false;
            r -= 1;
        } else if r == 0 && !f {
            f = true;
            r += 1;
        } else {
            if f {
                r += 1
            } else {
                r -= 1
            }
        }
    }

    String::from_utf8(res.into_iter().flatten().collect()).unwrap()
}

fn main() {
    dbg!(convert("PAYPALISHIRING".to_string(), 3));
    dbg!(convert("AB".to_string(), 1));
}
