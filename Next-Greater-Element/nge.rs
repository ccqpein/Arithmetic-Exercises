pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }

    let mut bucket: Vec<i32> = nums.iter().rev().map(|n| *n).collect();
    //dbg!(&bucket);
    let mut res = Vec::with_capacity(nums.len());
    for n in nums.iter().rev() {
        //dbg!(&res);
        loop {
            match bucket.pop() {
                Some(nn) => {
                    //dbg!(&nn);
                    if nn <= *n {
                        //bucket.pop();
                    } else {
                        bucket.push(nn);
                        bucket.push(*n);
                        res.push(Some(nn));
                        break;
                    }
                }
                None => {
                    res.push(None);
                    bucket.push(*n);
                    break;
                }
            }
        }
    }

    // bucket.clear();
    res.reverse();
    // for (ind, n) in nums.iter().enumerate() {
    //     loop {
    //         match bucket.pop() {
    //             Some(nn) => {
    //                 if nn <= *n {
    //                     bucket.pop();
    //                 } else {
    //                     if res[ind].is_some() {
    //                         *res[ind].as_mut().unwrap() = i32::max(nn, res[ind].unwrap());
    //                     } else {
    //                         res[ind] = Some(nn)
    //                     }
    //                     bucket.push(nn);
    //                     bucket.push(*n);
    //                     break;
    //                 }
    //             }
    //             None => {
    //                 bucket.push(*n);
    //                 break;
    //             }
    //         }
    //     }
    // }

    res.into_iter()
        .map(|a| match a {
            Some(aa) => aa,
            None => -1,
        })
        .collect()
}

fn main() {
    dbg!(next_greater_elements(vec![1, 2, 1]));
    dbg!(next_greater_elements(vec![5, 4, 3, 2, 1]));
}
