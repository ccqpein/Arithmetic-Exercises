fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut inner_num = nums.clone();
    inner_num.sort();

    let mut a = 0;
    let mut largest = 0;
    let mut last: i32 = 0;

    for i in inner_num {
        if a == 0 {
            a += 1;
            last = i;
            continue;
        }

        if i - last == 1 {
            a += 1;
        } else if i - last == 0 {

        } else {
            if a >= largest {
                largest = a;
            }
            a = 1
        }
        last = i
    }

    if a >= largest {
        largest = a;
    }
    largest
}

fn main() {}
