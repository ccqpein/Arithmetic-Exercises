use std::fmt::Debug;

fn bubble_sort<T>(a: &Vec<T>) -> Vec<T>
where
    T: PartialEq + PartialOrd + Clone,
{
    let length = a.len();
    let mut flag = true;
    let mut b = a.clone();
    while (flag) {
        flag = false;
        let mut temp: Vec<T> = vec![];
        let mut this = b[0].clone();
        for i in 1..length {
            if b[i] < this {
                temp.push(b[i].clone());
                flag = true;
            } else {
                temp.push(this);
                this = b[i].clone();
            }
        }
        temp.push(this);
        b = temp;
    }
    b
}

fn bubble_sort_2<T>(a: &Vec<T>) -> Vec<T>
where
    T: PartialEq + PartialOrd + Clone + Debug,
{
    let length = a.len();
    if length == 0 {
        return Vec::new();
    }
    let mut flag = true;
    let mut aa = a.clone();
    while flag {
        flag = false;
        let mut this = aa[0].clone();
        let mut temp_vec: Vec<T> = vec![];

        for i in aa.drain(1..) {
            if i < this {
                temp_vec.push(i.clone());
                flag = true
            } else {
                temp_vec.push(this);
                this = i;
            }
        }
        temp_vec.push(this);
        aa = temp_vec;
    }
    aa
}

fn main() {
    let mut test = vec![
        30, 63, 83, 40, 15, 45, 46, 15, 56, 39, 82, 97, 59, 88, 3, 1, 40, 95, 83, 32, 38, 70, 4,
        87, 54, 48, 19, 8, 52, 49, 64, 72, 46, 72, 59, 36, 21, 68, 81, 34, 23, 6, 70, 80, 80, 12,
        32, 84, 17, 19, 28, 58, 68, 19, 65, 46, 43, 22, 12, 95, 89, 15, 39, 88, 64, 95, 99, 25, 2,
        7, 86, 36, 73, 90, 30, 31, 0, 62, 73, 35, 4, 26, 0, 93, 91, 77, 34, 92, 31, 56, 34, 61, 23,
        47, 78, 5, 5, 26, 36, 71, 50, 5, 59, 22, 21, 0, 72, 72, 72, 69, 5, 11, 95, 5, 0, 14, 34,
        91, 4, 27, 46, 21, 94, 96, 48, 58, 79, 21, 65, 35, 17, 16, 57, 91, 36, 50, 16, 82, 92, 1,
        29, 52, 74, 90, 48, 79, 81, 53, 46, 82, 36, 43, 64, 24, 55, 48, 27, 21, 69, 93, 49, 70, 58,
        8, 50, 97, 30, 68, 1, 34, 15, 38, 52, 27, 50, 10, 22, 67, 25, 37, 84, 91, 13, 15, 0, 5, 31,
        18, 5, 31, 49, 93, 95, 3, 86, 11, 37, 68, 43, 74,
    ];
    assert_eq!(bubble_sort(&test), {
        let mut copy = test.clone();
        copy.sort();
        copy
    });

    assert_eq!(bubble_sort_2(&test), {
        let mut copy = test.clone();
        copy.sort();
        copy
    });
    assert_eq!(bubble_sort(&test), bubble_sort_2(&test));

    // test runtime cost
    use std::time::Instant;
    let start = Instant::now();
    for _ in 0..100 {
        bubble_sort(&test);
    }
    let duration = start.elapsed();
    println!("Time elapsed in bubble_sort() is: {:?}", duration);

    let start = Instant::now();
    for _ in 0..100 {
        bubble_sort_2(&test);
    }
    let duration = start.elapsed();
    println!("Time elapsed in bubble_sort_2() is: {:?}", duration);
}
