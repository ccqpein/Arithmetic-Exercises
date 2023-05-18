#[derive(Debug)]
enum Status {
    Up,
    Down,
    Flat,
}

pub fn longest_mountain(arr: Vec<i32>) -> i32 {
    let mut status = Status::Flat;
    let mut last = arr[0];
    let mut length = 0;
    let mut bucket = vec![];

    for i in arr {
        println!(
            "length: {}, bucket: {:?}, status: {:?}",
            length, bucket, status
        );
        match status {
            Status::Up => {
                if i > last {
                    length += 1;
                } else if i < last {
                    status = Status::Down;
                    length += 1;
                } else {
                    status = Status::Flat;
                    length = 0;
                }
            }
            Status::Down => {
                if i > last {
                    status = Status::Up;
                    bucket.push(length);
                    length = 2;
                } else if i < last {
                    length += 1;
                } else {
                    status = Status::Flat;
                    bucket.push(length);
                    length = 0;
                }
            }
            Status::Flat => {
                if i > last {
                    status = Status::Up;
                    length = 2;
                } else if i < last {
                    //status = Status::Down;
                }
            }
        }
        last = i
    }

    if let Status::Down = status {
        bucket.push(length);
    }

    bucket.into_iter().max().unwrap_or(0)
}

fn main() {
    assert_eq!(longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5);
    assert_eq!(longest_mountain(vec![2, 2, 2]), 0);
    assert_eq!(longest_mountain(vec![0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0]), 11);
    assert_eq!(longest_mountain(vec![2, 3]), 0);
    assert_eq!(longest_mountain(vec![875, 884, 239, 731, 723, 685]), 4);
}
