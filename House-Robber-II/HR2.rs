pub fn rob_trash(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    // let (mut max_ind, mut v) = (0usize, 0_i32);
    // for (ind, tempv) in nums.iter().enumerate() {
    //     if *tempv >= v {
    //         v = *tempv;
    //         max_ind = ind;
    //     }
    // }
    // if v == 0 {
    //     return 0;
    // }

    // let mut head = nums[max_ind + 1..nums.len()].to_vec();
    // let mut tail = nums[0..max_ind].to_vec();
    // let _ = head.append(&mut tail);

    // dbg!(&head);
    // if head.len() == 0 {
    //     return v;
    // }
    // return v + max_house(&head, 0, head.len() - 1);

    (0..nums.len())
        .collect::<Vec<usize>>()
        .iter()
        .map(|max_ind| {
            let v = nums[*max_ind as usize];
            let mut head = nums[max_ind + 1..nums.len()].to_vec();
            let mut tail = nums[0..*max_ind as usize].to_vec();
            let _ = head.append(&mut tail);
            //dbg!(&head);
            if head.len() == 0 {
                return v;
            };
            v + max_house(&head, 0, head.len() - 1)
        })
        .max()
        .unwrap()
}

//first step is nums, 0, nums.len()-1
pub fn max_house(nums: &Vec<i32>, head: usize, tail: usize) -> i32 {
    if tail - head <= 1 {
        return 0;
    }
    // let (mut max_ind, mut v) = (0usize, 0_i32);
    // for (ind, tempv) in nums[head + 1..tail].iter().enumerate() {
    //     if *tempv >= v {
    //         v = *tempv;
    //         max_ind = ind;
    //     }
    // }
    //dbg!(&v);

    (head + 1..tail)
        .collect::<Vec<usize>>()
        .iter()
        .map(|max_ind| {
            let v = nums[*max_ind as usize];
            dbg!(&v);
            v + max_house(nums, head, head + max_ind) + max_house(nums, head + 1 + max_ind, tail)
        })
        .max()
        .unwrap_or(0)

    //v + max_house(nums, head, head + max_ind) + max_house(nums, head + 2 + max_ind, tail)
}

/////code before are trash
////
////
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    if nums.len() <= 3 {
        return *nums.iter().max().unwrap();
    }

    *[
        max_inner(&nums[1..].to_vec()),
        max_inner(&nums[..nums.len() - 1].to_vec()),
    ]
    .iter()
    .max()
    .unwrap()
}

pub fn max_inner(nums: &Vec<i32>) -> i32 {
    let (mut before_max, mut now_max) = (0, 0);
    let mut temp: i32;
    for (_, tempv) in nums.iter().enumerate() {
        temp = now_max;
        now_max = *[before_max + tempv, now_max].iter().max().unwrap();
        before_max = temp;
    }
    now_max
}

fn main() {
    dbg!(rob(vec![1, 2, 3, 1])); //=> 4
    dbg!(rob(vec![]));
    dbg!(rob(vec![0]));
    dbg!(rob(vec![1]));
    dbg!(rob(vec![1, 7, 9, 4]));
    dbg!(rob(vec![2, 7, 9, 3, 1])); //=> 11
}
