pub fn dominant_index(nums: Vec<i32>) -> i32 {
    let (mut largest_ind, mut largest_v) = (0, 0);

    let mut ind = 0;
    let mut more_larger = false;
    for n in nums {
        if n >= 2 * largest_v {
            largest_ind = ind;
            largest_v = n;
            more_larger = false;
        } else if n > largest_v {
            more_larger = true;
            largest_v = n;
        } else {
            if 2 * n > largest_v {
                more_larger = true;
            }
        }
        ind += 1;
    }

    if more_larger {
        return -1;
    }
    largest_ind
}

fn main() {
    let test0 = vec![3, 6, 1, 0];
    dbg!(dominant_index(test0));

    let test1 = vec![1, 2, 3, 4];
    dbg!(dominant_index(test1));
}
