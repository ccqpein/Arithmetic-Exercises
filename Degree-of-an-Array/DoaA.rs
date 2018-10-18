use std::collections::HashMap;

fn find_shortest_subarray(nums: Vec<u32>) -> u32 {
    //tuple of top degree, and which numbers has this degree.
    let mut nums_degree: (u32, Vec<u32>) = (0, vec![]);
    //key is number, value is degree and index of first and last index of this number
    let mut nums_index: HashMap<u32, (u32, (usize, usize))> = HashMap::new();

    for (ind, num) in nums.iter().enumerate() {
        if nums_index.contains_key(&num) {
            let mut val = nums_index.get_mut(num).unwrap();
            val.0 += 1;
            (val.1).1 = ind;
        } else {
            nums_index.insert(*num, (1, (ind, 0)));
        }
    }

    //update nums_degree
    for (num, detls) in &nums_index {
        if detls.0 < nums_degree.0 {
            continue;
        }

        if detls.0 > nums_degree.0 {
            nums_degree = (detls.0, vec![*num]);
        } else if detls.0 == nums_degree.0 {
            nums_degree.1.push(*num);
        }
    }

    //println!("{:?}", nums_degree);

    let mut result: Vec<_> = nums_degree
        .1
        .into_iter()
        .map(|x| {
            let a = nums_index[&x].1;
            //println!("{:?}", a);
            return a.1 - a.0;
        })
        .collect();

    result.sort();
    result[0] as u32 + 1
}

fn main() {
    let testcase0 = vec![1, 2, 2, 3, 1];
    let testcase1 = vec![1, 2, 2, 3, 1, 4, 2];

    println!("{:?}", find_shortest_subarray(testcase0));
    println!("{:?}", find_shortest_subarray(testcase1));
}
