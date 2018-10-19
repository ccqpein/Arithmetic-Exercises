use std::collections::HashMap;

fn distribute_candies(nums: Vec<i32>) -> u32 {
    let mut store_temp: HashMap<i32, u32> = HashMap::new();
    let len_nums = nums.len();

    for num in nums {
        if store_temp.contains_key(&num) {
            *store_temp.get_mut(&num).unwrap() += 1;
        } else {
            store_temp.insert(num, 1);
        }
    }

    let kinds_num = store_temp.keys().count();
    if kinds_num > len_nums / 2 {
        return (len_nums / 2) as u32;
    }

    kinds_num as u32
}

fn main() {
    let testcase0 = vec![1, 1, 2, 2, 3, 3];
    let testcase1 = vec![1, 1, 2, 3];

    println!("{:?}", distribute_candies(testcase0));
    println!("{:?}", distribute_candies(testcase1));
}
