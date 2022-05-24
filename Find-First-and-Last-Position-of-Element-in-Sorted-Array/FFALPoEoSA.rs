pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![-1, -1];
    }

    if nums.len() == 1 {
        if nums[0] == target {
            return vec![0, 0];
        } else {
            return vec![-1, -1];
        }
    }
    vec![
        find_first(&nums, 0, nums.len() - 1, target),
        find_end(&nums, 0, nums.len() - 1, target),
    ]
}

fn find_first(nums: &[i32], start: usize, end: usize, target: i32) -> i32 {
    //println!("{}, {}", start, end);

    let mid_end = (start + end) / 2;
    //println!("{}", mid_end);
    if mid_end == start {
        if nums[start] == target {
            return start as i32;
        } else if nums[end] == target {
            return end as i32;
        } else {
            return -1;
        }
    }

    if nums[mid_end] < target {
        find_first(nums, mid_end, end, target)
    } else if nums[mid_end] == target {
        find_first(nums, start, mid_end, target)
    } else {
        find_first(nums, start, mid_end, target)
    }
}

fn find_end(nums: &[i32], start: usize, end: usize, target: i32) -> i32 {
    //println!("{}, {}", start, end);

    let mid_end = (start + end) / 2;
    //println!("{}", mid_end);
    if mid_end == start {
        if nums[end] == target {
            return end as i32;
        } else if nums[start] == target {
            return start as i32;
        } else {
            return -1;
        }
    }

    if nums[mid_end] < target {
        find_end(nums, mid_end + 1, end, target)
    } else if nums[mid_end] == target {
        find_end(nums, mid_end, end, target)
    } else {
        find_end(nums, start, mid_end, target)
    }
}

fn main() {
    //dbg!(find_first(&vec![5, 7, 7, 8, 8, 10], 0, 5, 6)); // -1
    //dbg!(find_first(&vec![5, 7, 7, 8, 8, 10], 0, 5, 7)); // 1
    //dbg!(find_first(&vec![5, 7, 7, 8, 8, 10], 0, 5, 8)); // 3
    //dbg!(find_end(&vec![5, 7, 7, 8, 8, 10], 0, 5, 8)); // 4

    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    assert_eq!(search_range(vec![], 0), vec![-1, -1]);
    assert_eq!(search_range(vec![1], 0), vec![-1, -1]);
    assert_eq!(search_range(vec![2, 2], 3), vec![-1, -1]);
}
