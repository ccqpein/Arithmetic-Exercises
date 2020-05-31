pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.len() == 0 {
        return vec![];
    }

    let (mut start, mut end) = (nums[0], nums[0]);
    let mut result: Vec<String> = vec![];

    for i in nums {
        if i > end + 1 {
            if start != end {
                result.push(vec![start.to_string(), end.to_string()].join("->"));
            } else {
                result.push(String::from(start.to_string()));
            }
            start = i;
        }
        end = i;
    }

    if start != end {
        result.push(vec![start.to_string(), end.to_string()].join("->"));
    } else {
        result.push(String::from(start.to_string()));
    }
    result
}

fn main() {
    assert_eq!(
        summary_ranges(vec![0, 1, 2, 4, 5, 7]),
        vec!["0->2", "4->5", "7"]
    );

    assert_eq!(
        summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
        vec!["0", "2->4", "6", "8->9"]
    );

    assert_eq!(summary_ranges(vec![]), Vec::new() as Vec<String>);
}
