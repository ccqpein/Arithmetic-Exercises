use std::{
    collections::HashSet,
    iter::{empty, once},
};

fn optimized_sieve(limit: usize) -> Box<dyn Iterator<Item = usize>> {
    if limit < 3 {
        return if limit < 2 {
            Box::new(empty())
        } else {
            Box::new(once(2))
        };
    }

    let ndxlmt = (limit - 3) / 2 + 1;
    let bfsz = ((limit - 3) / 2) / 32 + 1;
    let mut cmpsts = vec![0u32; bfsz];
    let sqrtndxlmt = ((limit as f64).sqrt() as usize - 3) / 2 + 1;

    for ndx in 0..sqrtndxlmt {
        if (cmpsts[ndx >> 5] & (1u32 << (ndx & 31))) == 0 {
            let p = ndx + ndx + 3;
            let mut cullpos = (p * p - 3) / 2;
            while cullpos < ndxlmt {
                unsafe {
                    // avoids array bounds check, which is already done above
                    let cptr = cmpsts.get_unchecked_mut(cullpos >> 5);
                    *cptr |= 1u32 << (cullpos & 31);
                }
                //                cmpsts[cullpos >> 5] |= 1u32 << (cullpos & 31); // with bounds check
                cullpos += p;
            }
        }
    }

    Box::new((-1..ndxlmt as isize).into_iter().filter_map(move |i| {
        if i < 0 {
            Some(2)
        } else {
            if cmpsts[i as usize >> 5] & (1u32 << (i & 31)) == 0 {
                Some((i + i + 3) as usize)
            } else {
                None
            }
        }
    }))
}

// fn give_me_e(n: f64) -> f64 {
//     (1_f64 + 1_f64 / n).powf(n)
// }

//const E: &str = include_str!("../data/big_vec");

fn main() {
    //println!("{:?}", optimized_sieve(10_000_000_000).last());
    let E: &str = include_str!("../data/big_e");
    let all_10_ds = optimized_sieve(10_000_000_000)
        .map(|n| n.to_string())
        .filter(|n| n.len() == 10)
        .collect::<HashSet<_>>();

    for i in (0..E.len() - 10) {
        if all_10_ds.contains(&E[i..i + 10]) {
            println!("is: {}, bit is {}", &E[i..i + 10], i);
            return;
        }
    }
}
