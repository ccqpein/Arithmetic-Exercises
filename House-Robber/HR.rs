use std::collections::HashMap;

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut table: HashMap<usize, i32> = HashMap::new();
    rob_inner(&nums, 0, nums.len(), &mut table)
}

fn rob_inner(nums: &Vec<i32>, start: usize, len: usize, ta: &mut HashMap<usize, i32>) -> i32 {
    if let Some(v) = ta.get(&start) {
        return *v;
    }

    use std::cmp::max;
    if start == len - 1 {
        ta.insert(start, nums[start]);
        return nums[start];
    } else if start == len {
        ta.insert(start, 0);
        return 0;
    }

    let a = max(
        (start + 1..len)
            .into_iter()
            .map(|x| rob_inner(nums, x, len, ta))
            .max()
            .unwrap(),
        nums[start] + rob_inner(nums, start + 2, len, ta),
    );

    ta.insert(start, a);

    a
}

fn main() {
    assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(rob(vec![2, 7, 9, 3, 1, 8]), 19);
    assert_eq!(
        rob(vec![
            183, 219, 57, 193, 94, 233, 202, 154, 65, 240, 97, 234, 100, 249, 186, 66, 90, 238,
            168, 128, 177, 235, 50, 81, 185, 165, 217, 207, 88, 80, 112, 78, 135, 62, 228, 247,
            211
        ]),
        3365
    );
    assert_eq!(
        rob(vec![9, 297, 196, 336, 435, 2, 343, 159, 146, 359, 45, 470, 265, 131, 17, 271, 74, 242, 448, 402, 55, 423, 414, 240, 430, 135, 322, 468, 422, 351, 441, 463, 30, 399, 132, 439, 463, 260, 399, 32, 374, 383, 276, 166, 104, 315, 314, 445, 458, 422, 104, 251, 382, 230, 484, 127, 355, 332, 317, 362, 257, 493, 474, 401, 40, 93, 433, 464, 136, 342, 98, 159, 223, 110, 89, 47, 53, 179, 219, 314, 486, 301, 307, 453, 37, 366, 334, 355, 26, 484, 124, 408, 346, 133, 420, 280, 124, 210, 358, 140,9, 297, 196, 336, 435, 2, 343, 159, 146, 359, 45, 470, 265, 131, 17, 271, 74, 242, 448, 402, 55, 423, 414, 240, 430, 135, 322, 468, 422, 351, 441, 463, 30, 399, 132, 439, 463, 260, 399, 32, 374, 383, 276, 166, 104, 315, 314, 445, 458, 422, 104, 251, 382, 230, 484, 127, 355, 332, 317, 362, 257, 493, 474, 401, 40, 93, 433, 464, 136, 342, 98, 159, 223, 110, 89, 47, 53, 179, 219, 314, 486, 301, 307, 453, 37, 366, 334, 355, 26, 484, 124, 408, 346, 133, 420, 280, 124, 210, 358, 140,9, 297, 196, 336, 435, 2, 343, 159, 146, 359, 45, 470, 265, 131, 17, 271, 74, 242, 448, 402, 55, 423, 414, 240, 430, 135, 322, 468, 422, 351, 441, 463, 30, 399, 132, 439, 463, 260, 399, 32, 374, 383, 276, 166, 104, 315, 314, 445, 458, 422, 104, 251, 382, 230, 484, 127, 355, 332, 317, 362, 257, 493, 474, 401, 40, 93, 433, 464, 136, 342, 98, 159, 223, 110, 89, 47, 53, 179, 219, 314, 486, 301, 307, 453, 37, 366, 334, 355, 26, 484, 124, 408, 346, 133, 420, 280, 124, 210, 358, 140]),
        46896
    );
}