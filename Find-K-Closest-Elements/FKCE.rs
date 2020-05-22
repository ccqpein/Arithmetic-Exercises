fn find_k_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut i_large = 0;
    let length = arr.len() as i32;
    for v in &arr {
        if *v >= x {
            break;
        } else {
            i_large += 1;
        }
    }

    let i = if i_large - k >= 0 { i_large - k } else { 0 };
    let j = if i_large + k < length {
        i_large + k
    } else {
        length
    };

    let mut temp_result: Vec<i32> = vec![];
    for ind in i..j {
        temp_result.push(*arr.iter().nth(ind as usize).unwrap());
    }

    //println!("{:?}", temp_result);
    temp_result.sort_by_key(|v| (*v - x).abs());
    let mut re = temp_result.drain(..k as usize).collect::<Vec<i32>>();
    re.sort();

    re
}

// faster version
fn find_closest_elements(mut arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut larger = arr
        .iter()
        .enumerate()
        .find(|(_, &e)| e >= x)
        .unwrap_or((arr.len(), &0))
        .0;

    let smaller = larger;

    if smaller == 0 {
        return arr.drain(smaller..smaller + (k as usize)).collect();
    }

    if larger == arr.len() {
        return arr.drain((larger - k as usize)..).collect();
    }

    let mut smaller_i32 = (smaller - 1) as i32;
    let mut count = 0;
    loop {
        //check here
        if smaller_i32 < 0 {
            return arr.drain(0..k as usize).collect();
        }

        if larger == arr.len() {
            return arr.drain((larger - k as usize)..).collect();
        }

        //
        if count == k {
            break;
        }

        if x - arr[smaller_i32 as usize] <= arr[larger] - x {
            smaller_i32 -= 1;
        } else if x - arr[smaller_i32 as usize] > arr[larger] - x {
            larger += 1;
        }
        count += 1;
    }

    arr.drain(smaller_i32 as usize + 1..larger).collect()
}

fn main() {
    assert_eq!(
        find_k_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
        vec![1, 2, 3, 4]
    );

    assert_eq!(
        find_k_closest_elements(vec![0, 0, 0, 1, 3, 5, 6, 7, 8, 8], 2, 2),
        vec![1, 3]
    );

    assert_eq!(
        find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
        vec![1, 2, 3, 4]
    );

    assert_eq!(
        find_closest_elements(vec![0, 0, 0, 1, 3, 5, 6, 7, 8, 8], 2, 2),
        vec![1, 3]
    );

    assert_eq!(
        find_closest_elements(vec![0, 1, 2, 2, 2, 3, 6, 8, 8, 9], 5, 9),
        vec![3, 6, 8, 8, 9]
    );
}
