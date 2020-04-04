// fn main() {
//     let nums = vec![0,1,0,3,12];
//     let mut result:Vec<i32> = Vec::new();
//     let mut zeros = 0;

//     for i in &nums {
//         if *i != 0 {
//             result.push(*i);
//         }else {
//             zeros += 1;
//         }
//     }

//     for _ in 0..zeros {
//         result.push(0)
//     }

//     println!("{:?}", nums);
//     println!("{:?}", result);
// }

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut result: Vec<i32> = Vec::new();
    let mut zeros = 0;

    for i in nums.clone() {
        if i != 0 {
            result.push(i);
        } else {
            zeros += 1;
        }
    }

    for _ in 0..zeros {
        result.push(0)
    }

    *nums = result;
}

pub fn move_zeroes_new(nums: &mut Vec<i32>) {
    let mut result: Vec<i32> = Vec::new();
    let mut zeros = 0;

    let len = nums.len().clone();

    for i in 0..len {
        if nums[i] != 0 {
            result.push(nums[i].clone());
        } else {
            zeros += 1;
        }
    }

    for _ in 0..zeros {
        result.push(0)
    }

    *nums = result;
}

fn main() {
    let mut testcase = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut testcase);

    println!("{:?}", testcase);
}
