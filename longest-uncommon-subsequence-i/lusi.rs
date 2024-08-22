// pub fn find_lu_slength(a: String, b: String) -> i32 {
//     let mut res = -1;
//     let mut cache = 0;
//     for (aa, bb) in a.chars().zip(b.chars()) {
//         if aa != bb {
//             cache += 1;
//         } else if cache != 0 {
//             if cache >= res {
//                 res = cache;
//             }
//             cache = 0;
//         }
//     }

//     if cache != 0 && cache > res {
//         res = cache;
//     }
//     res
// }

pub fn find_lu_slength(a: String, b: String) -> i32 {
    if a == b {
        -1
    } else {
        a.len().max(b.len()) as i32
    }
}

fn main() {
    assert_eq!(3, find_lu_slength("aba".to_string(), "cdc".to_string()));
    assert_eq!(3, find_lu_slength("aaa".to_string(), "bbb".to_string()));
    assert_eq!(-1, find_lu_slength("aaa".to_string(), "aaa".to_string()));
}
