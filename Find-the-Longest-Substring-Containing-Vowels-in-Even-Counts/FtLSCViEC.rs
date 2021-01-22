// pub fn find_the_longest_substring(s: String) -> i32 {
//     use std::collections::VecDeque;
//     let mut cache: VecDeque<(char, usize)> = VecDeque::new();

//     let mut largest_length = 0;
//     for (ind, c) in s.chars().enumerate() {
//         match c {
//             'a' | 'e' | 'i' | 'o' | 'u' => {
//                 // check if has two already
//                 if cache.iter().filter(|(ch, _)| *ch == c).count() == 2 {
//                     let now_length = ind - cache[0].1 + 1;
//                     if now_length > largest_length {
//                         largest_length = now_length
//                     }
//                     //println!("{:?}", cache);
//                     // pop out if it is the first one
//                     loop {
//                         if let Some(d) = cache.pop_front() {
//                             // if first one in cache is this one
//                             if d.0 == c {
//                                 cache.push_back((c, ind));
//                                 break;
//                             }
//                         } else {
//                             break;
//                         }
//                     }
//                 } else {
//                     // push to cache
//                     cache.push_back((c, ind))
//                 }
//             }
//             _ => {}
//         }
//     }
//     largest_length as i32
// }

// use std::collections::VecDeque;

// pub fn find_the_longest_substring(s: String) -> i32 {
//     let mut cache: VecDeque<(char, usize)> = VecDeque::new();
//     s.chars().enumerate().for_each(|(ind, v)| {
//         if ['a', 'e', 'i', 'o', 'u'].contains(&v) {
//             cache.push_back((v, ind))
//         }
//     });

//     println!("{:?}", cache);
//     let mut largest = 0;
//     let mut pp = 0;
//     loop {
//         let first = cache[pp].0;
//         for ind in pp + 1..cache.len() {
//             if cache[ind].0 == first {
//                 println!("ind: {:?}", ind);
//                 if match_two(&cache, ind) {
//                     if (cache[ind].1 - cache[0].1) > largest {
//                         //println!("ind: {:?}", ind);
//                         largest = cache[ind].1 - cache[0].1;
//                         println!("largest: {:?}", largest);
//                     }
//                 }
//             }
//         }
//         //cache.pop_front();
//         if cache.len() - 1 == pp {
//             break;
//         }
//         pp += 1;
//     }
//     largest as i32
// }

// fn match_two(v: &VecDeque<(char, usize)>, ind: usize) -> bool {
//     println!("{:?}", v);
//     let mut aa = [0; 5];
//     for i in 0..ind {
//         match v[i].0 {
//             'a' => aa[0] += 1,
//             'e' => aa[1] += 1,
//             'i' => aa[2] += 1,
//             'o' => aa[3] += 1,
//             'u' => aa[4] += 1,
//             _ => (),
//         }
//     }

//     aa.iter().all(|&v| v == 0 || v == 2)
// }

pub fn find_the_longest_substring(s: String) -> i32 {
    let mut aeiou = [0; 5];
    let mut cache = vec![(0, aeiou.clone())];
    for (ind, c) in s.chars().enumerate() {
        match c {
            'a' => aeiou[0] += 1,
            'e' => aeiou[1] += 1,
            'i' => aeiou[2] += 1,
            'o' => aeiou[3] += 1,
            'u' => aeiou[4] += 1,
            _ => {}
        }
        cache.push((ind + 1, aeiou.clone()))
    }

    //dbg!(&cache);
    let mut largest = 0;
    for i in 0..cache.len() {
        for j in (i + 1..cache.len()).rev() {
            if match_two(&cache, i, j) {
                if cache[j].0 - cache[i].0 > largest {
                    largest = cache[j].0 - cache[i].0
                } else {
                    break;
                }
            }
        }
    }

    largest as i32
}

fn match_two(cache: &Vec<(usize, [i32; 5])>, i: usize, j: usize) -> bool {
    let mut re = [0; 5];
    for x in 0..5 {
        re[x] = cache[j].1[x] - cache[i].1[x]
    }
    re.iter().all(|&v| v % 2 == 0)
}

fn main() {
    assert_eq!(
        find_the_longest_substring("eleetminicoworoep".to_string()),
        13
    );
    assert_eq!(find_the_longest_substring("leetcodeisgreat".to_string()), 5);
    assert_eq!(find_the_longest_substring("bcbcbc".to_string()), 6);
    assert_eq!(find_the_longest_substring("id".to_string()), 1);
    assert_eq!(find_the_longest_substring("qnplnlarrtztkotazhufrsfczrzibvccaoayyihidztfljcffiqfviuwjowkppdajmknzgidixqgtnahamebxfowqvnrhuzwqohquamvszkvunbxjegbjccjjxfnsiearbsgsofywtqbmgldgsvnsgpdvmjqpaktmjafgkzszekngivdmrlvrpyrhcxbceffrgiyktqilkkdjhtywpesrydkbncmzeekdtszmcsrhsciljsrdoidzbjatvacndzbghzsnfdofvhfxdnmzrjriwpkdgukbaazjxtkomkmccktodig".to_string()), 295);
}
