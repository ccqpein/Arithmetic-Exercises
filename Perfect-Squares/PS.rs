fn make_squares_list(n: i32) -> Vec<i32> {
    let mut re: Vec<i32> = vec![];
    for i in 0..n + 1 {
        let temp = i * i;
        if temp > n {
            break;
        } else {
            re.push(temp);
        }
    }

    re
}

fn sum_list(l1: &Vec<i32>, l2: &Vec<i32>) -> Vec<i32> {
    let mut temp: Vec<Vec<i32>> = l1
        .iter()
        .map(|a| l2.iter().map(|b| b + a).collect())
        .collect();

    let mut re: Vec<i32> = vec![];
    for i in temp.iter_mut() {
        re.append(i);
    }

    re
}

fn gen_all_nums(time: &mut u32, basic: &Vec<i32>) -> Vec<i32> {
    let mut re: Vec<i32> = sum_list(basic, basic);
    while (*time > 2) {
        let temp = re.clone();
        re.append(&mut sum_list(basic, &temp));
        *time -= 1;
        //println!("{:?}", re);
    }
    re
}

fn ps(n: i32) -> i32 {
    let basic_list = make_squares_list(n);

    for i in 2..4 {
        if gen_all_nums(&mut (i as u32), &basic_list).contains(&n) {
            return i;
        }
    }

    0
}

fn main() {
    //println!("{:?}", make_squares_list(5));
    //println!("{:?}", sum_list(&vec![1, 2, 3], &vec![4, 5, 6]));
    //println!("{:?}", gen_all_nums(&mut (2 as u32), &vec![1, 2]));

    println!("{:?}", ps(13)); //=> 2
    println!("{:?}", ps(3)); //=> 3
    println!("{:?}", ps(12)); //=> 3
}
