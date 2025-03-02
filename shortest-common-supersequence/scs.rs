use std::collections::HashMap;

pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    // let a = helper(str1.as_bytes(), str2.as_bytes());
    // let b = helper(str2.as_bytes(), str1.as_bytes());

    // if a.len() < b.len() {
    //     String::from_utf8(a).unwrap()
    // } else {
    //     String::from_utf8(b).unwrap()
    // }
    let mut cache = HashMap::new();
    String::from_utf8(shortest(str1.as_bytes(), str2.as_bytes(), &mut cache)).unwrap()
}

fn shortest(s1: &[u8], s2: &[u8], cache: &mut HashMap<(Vec<u8>, Vec<u8>), Vec<u8>>) -> Vec<u8> {
    //use std::collections::HashMap;
    //let mut cache = HashMap::new();

    if let Some(re) = cache.get(&(s1.to_vec(), s2.to_vec())) {
        return re.to_vec();
    }

    if let Some(re) = cache.get(&(s2.to_vec(), s1.to_vec())) {
        return re.to_vec();
    }

    let a = helper(s1, s2, cache);
    let b = helper(s2, s1, cache);

    if a.len() < b.len() {
        cache.insert((s1.to_vec(), s2.to_vec()), a.clone());
        cache.insert((s2.to_vec(), s1.to_vec()), a.clone());
        a
    } else {
        cache.insert((s1.to_vec(), s2.to_vec()), b.clone());
        cache.insert((s2.to_vec(), s1.to_vec()), b.clone());
        b
    }
}

fn helper(s1: &[u8], s2: &[u8], cache: &mut HashMap<(Vec<u8>, Vec<u8>), Vec<u8>>) -> Vec<u8> {
    if s2.len() == 0 {
        return s1.to_vec();
    }
    if s1.len() == 0 {
        return s2.to_vec();
    }

    if let Some(re) = cache.get(&(s1.to_vec(), s2.to_vec())) {
        return re.to_vec();
    }

    if let Some(re) = cache.get(&(s2.to_vec(), s1.to_vec())) {
        return re.to_vec();
    }

    let mut len = s1.len() + s2.len();
    let mut min_re = {
        let mut a = s1.to_vec();
        a.append(&mut s2.to_vec());
        a
    };

    for s1ind in 0..s1.len() {
        if s1[s1ind] == s2[0] {
            let mut a = compare2(s1.get(s1ind..).unwrap(), s2, cache);
            // dbg!(s1ind);
            // dbg!(&a);
            // dbg!(s1);
            // dbg!(s2);
            if s1ind + a.len() < len {
                len = a.len() + s1ind;
                min_re = {
                    let mut q = s1.get(0..s1ind).unwrap().to_vec();
                    q.append(&mut a);
                    q
                }
            }
        } else {
            let mut a = s1.get(0..s1ind).unwrap().to_vec();
            a.append(&mut vec![s1[s1ind], s2[0]]);
            a.append(&mut helper(
                s1.get(s1ind + 1..).unwrap(),
                s2.get(1..).unwrap(),
                cache,
            ));
            if a.len() < len {
                len = a.len();
                min_re = a;
            }
        }
    }

    min_re
}

// fn compare<'a>(s1: &'a [u8], s2: &'a [u8]) -> (bool, &'a [u8]) {
//     let mut ind = 0;
//     loop {
//         match (s1.get(ind), s2.get(ind)) {
//             (None, None) => return (true, s1),
//             (None, Some(_)) => return (true, s2),
//             (Some(_), None) => return (true, s1),
//             (Some(a), Some(b)) => {
//                 if a != b {
//                     return (false, s1);
//                 }
//             }
//         }
//         ind += 1
//     }
// }

fn compare2<'a>(
    s1: &'a [u8],
    s2: &'a [u8],
    cache: &mut HashMap<(Vec<u8>, Vec<u8>), Vec<u8>>,
) -> Vec<u8> {
    let mut ind = 0;
    loop {
        match (s1.get(ind), s2.get(ind)) {
            (None, None) => return s1.to_vec(),
            (None, Some(_)) => return s2.to_vec(),
            (Some(_), None) => return s1.to_vec(),
            (Some(a), Some(b)) => {
                if a != b {
                    let mut a = s1.get(0..ind).unwrap().to_vec();
                    a.append(&mut shortest(
                        s1.get(ind..).unwrap(),
                        s2.get(ind..).unwrap(),
                        cache,
                    ));
                    // let mut b = helper(s1.get(ind..).unwrap(), s2.get(ind..).unwrap());
                    // let mut c = helper(s2.get(ind..).unwrap(), s1.get(ind..).unwrap());
                    // if b.len() < c.len() {
                    //     a.append(&mut b);
                    // } else {
                    //     a.append(&mut c);
                    // }
                    return a;
                }
            }
        }
        ind += 1
    }
}

fn main() {
    // assert_eq!(
    //     "cabac".to_string(),
    //     shortest_common_supersequence("abac".to_string(), "cab".to_string())
    // );

    // assert_eq!(
    //     "aaaaaaaa".to_string(),
    //     shortest_common_supersequence("aaaaaaaa".to_string(), "aaaaaaaa".to_string())
    // );

    // assert_eq!(
    //     "bbbaaababbb".to_string(),
    //     shortest_common_supersequence("bbbaaaba".to_string(), "bbababbb".to_string())
    // );

    // dbg!(String::from_utf8(helper(
    //     "baaaba".as_bytes(),
    //     "ababbb".as_bytes()
    // )));
    let mut cache = HashMap::new();
    dbg!(String::from_utf8(helper(
        "ba".as_bytes(),
        "bbb".as_bytes(),
        &mut cache
    )));
    //dbg!(String::from_utf8(helper("a".as_bytes(), "bb".as_bytes())));

    //dbg!(String::from_utf8(helper("bc".as_bytes(), "aca".as_bytes())));

    dbg!(String::from_utf8(shortest(
        "bcaaacbbbcbdcaddadcacbdddcdcccdadadcbabaccbccdcdcbcaccacbbdcbabb".as_bytes(),
        "dddbbdcbccaccbababaacbcbacdddcdabadcacddbacadabdabcdbaaabaccbdaa".as_bytes(),
        &mut cache
    )));
}
