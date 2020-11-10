pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // PROBLEM here
    let mut bucket: [(i32, i32); 2147483647] = [(0, 0); 2147483647]; // [(start(0)|end(1), height), length]
    for building in buildings {
        bucket[building[0] as usize] = (0, building[2]);
        bucket[building[1] as usize] = (1, building[2]);
    }

    let mut result: Vec<Vec<i32>> = vec![];
    let mut last: (i32, i32) = (1, 0);
    for (i, d) in bucket.iter().enumerate() {
        if *d == (0, 0) {
            continue;
        }

        match (d.0, last.0) {
            // this is start, last is end
            (0, 1) => result.push(vec![i as i32, d.1]),

            // this is end, last is start
            (1, 0) => {
                if d.1 > last.1 {
                    result.push(vec![i as i32, last.1]);
                } else if d.1 == last.1 {
                    result.push(vec![i as i32, 0])
                }
            }

            // this is start, last is start
            (0, 0) => {
                if d.1 > last.1 {
                    result.push(vec![i as i32, d.1]);
                }
            }

            // this is end, last is end
            (1, 1) => {
                if let Some(e) = result.last() {
                    if e[1] == 0 {
                        result.pop();
                    }
                }
                result.push(vec![i as i32, 0])
            }

            _ => (),
        }
        last = d.clone();
    }

    result
}

fn main() {
    dbg!(get_skyline(vec![
        vec![2, 9, 10],
        vec![3, 7, 15],
        vec![5, 12, 12],
        vec![15, 20, 10],
        vec![19, 24, 8]
    ]));

    assert_eq!(
        vec![vec![0, 3], vec![1, 0]],
        get_skyline(vec![vec![0, 1, 3]])
    );
}
