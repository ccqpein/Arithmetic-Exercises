pub fn triangle_number(nums: Vec<i32>) -> i32 {
    //let mut nums = nums.clone();
    //nums.sort();
    //const value_max: usize = 80;
    const value_max: usize = 1001;
    let mut fre = [None; value_max];
    for n in nums {
        match fre.get_mut(n as usize).unwrap() {
            Some(a) => *a += 1,
            x @ None => *x = Some(1),
        };
    }

    let mut res = 0;

    // n is the value, 3 elements
    for n in 1..value_max {
        let Some(fre_n) = fre[n as usize] else {
            continue;
        };

        if fre_n >= 3 {
            res += (fre_n * (fre_n - 1) * (fre_n - 2)) / (3 * 2)
        }

        for m in n + 1..value_max {
            let Some(fre_m) = fre[m as usize] else {
                continue;
            };
            for z in m + 1..value_max {
                //let fre_z = fre[z as usize];
                let Some(fre_z) = fre[z as usize] else {
                    continue;
                };
                if z < n + m {
                    res += fre_n * fre_m * fre_z;
                }
            }
        }
    }

    for n in 1..value_max {
        let Some(fre_n) = fre[n as usize] else {
            continue;
        };
        for m in n + 1..value_max {
            let Some(fre_m) = fre[m as usize] else {
                continue;
            };

            if (fre_n + fre_m >= 3) {
                if fre_n == 1 {
                    res += (fre_m * (fre_m - 1)) / 2;
                    continue;
                }

                if fre_m == 1 {
                    if m >= n * 2 {
                        continue;
                    } else {
                        res += (fre_n * (fre_n - 1) / 2) * fre_m;
                        continue;
                    }
                }

                // if fre_m >= 2 {
                //     res += ((fre_m * (fre_m - 1)) / 2) * fre_n;
                //     continue;
                // }

                if m >= n * 2 {
                    res += ((fre_m * (fre_m - 1)) / 2) * fre_n;
                    continue;
                }

                if m < n * 2 {
                    res += (fre_n * (fre_n - 1) / 2) * fre_m;
                    res += (fre_m * (fre_m - 1) / 2) * fre_n;
                }
            }
        }
    }

    res
}

// fn helper(a: i32) -> i32 {
//     (a * (a - 1) * (a - 2)) / (3 * 2)
// }

fn main() {
    //assert_eq!(triangle_number(vec![2, 2, 3, 4]), 3);
    //assert_eq!(triangle_number(vec![4, 2, 3, 4]), 4);
    //assert_eq!(triangle_number(vec![0, 0, 0]), 0);
    //assert_eq!(triangle_number(vec![2, 3, 4]), 1);

    assert_eq!(
        triangle_number(vec![16, 10, 10, 13, 76, 64, 71, 71, 12, 33]),
        47
    );

    assert_eq!(triangle_number(vec![54, 98, 9, 98, 5, 61, 54, 83]), 26);
}
