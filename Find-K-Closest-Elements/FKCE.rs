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

fn main() {
    println!("{:?}", find_k_closest_elements(vec![1, 2, 3, 4, 5], 4, 3));
    println!(
        "{:?}",
        find_k_closest_elements(vec![0, 0, 0, 1, 3, 5, 6, 7, 8, 8], 2, 2)
    );
}
