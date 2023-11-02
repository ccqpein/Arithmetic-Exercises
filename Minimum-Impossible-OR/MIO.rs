pub fn min_impossible_or_bkp(nums: Vec<i32>) -> i32 {
    let mut nums = nums.into_iter().map(|a| a as u32).collect::<Vec<_>>();
    nums.sort();

    let mut result = 0;
    let mut tz = 0;
    for ind in 0..nums.len() {
        //dbg!(nums[ind]);
        result |= nums[ind];

        let lz = result.leading_zeros();
        //dbg!(lz);

        let x = result ^ u32::MAX;
        let lo = x.leading_ones();
        tz = x.trailing_zeros();

        //println!("{:#b}", (result ^ u32::MAX));
        //println!("{:#b}", result);
        //println!("{:#b}", nums[ind]);
        //dbg!(lo);
        //dbg!(tz);

        if lz == lo && lo + tz != 32 {
            //dbg!((2 ^ (32 - lo)));
            return (2_i32.pow(tz)) as i32;
        }
    }

    //dbg!(result);
    (2_i32.pow(tz)) as i32
}

pub fn min_impossible_or(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut s = HashSet::new();

    for n in nums {
        if n == (n & -n) {
            s.insert(n);
        }
    }

    let mut i = 1;
    while s.contains(&i) {
        i = i << 1
    }

    i
}

fn main() {
    //println!("{:#b}", (11 ^ i32::MAX));
    //dbg!((11_i8 & 0).leading_zeros());

    //dbg!(min_impossible_or(vec![2, 1])); // 4
    //dbg!(min_impossible_or(vec![5, 3, 2])); // 1
    dbg!(min_impossible_or(vec![4, 32, 16, 8, 8, 75, 1, 2])); // 64
}
