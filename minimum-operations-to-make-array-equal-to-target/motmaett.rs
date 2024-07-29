fn chunk_vec(detla: Vec<i32>) -> Vec<Vec<i32>> {
    dbg!(&detla);
    let mut cache = vec![];
    let mut flag = if detla[0] >= 0 { true } else { false };
    let mut result = vec![];
    for n in detla {
        match flag {
            true if n >= 0 => cache.push(n),
            true if n < 0 => {
                result.push(cache);
                cache = vec![];
                flag = false;
                cache.push(n)
            }
            false if n >= 0 => {
                result.push(cache);
                cache = vec![];
                flag = true;
                cache.push(n)
            }
            false if n < 0 => cache.push(n),
            _ => unreachable!(),
        }
    }

    if cache.len() > 0 {
        result.push(cache)
    }

    result
        .into_iter()
        .map(|a| {
            a.split(|x| *x == 0)
                .into_iter()
                .map(|s| s.to_vec())
                .filter(|a| a.len() > 0)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

pub fn minimum_operations_bkp(nums: Vec<i32>, target: Vec<i32>) -> i64 {
    let detla: Vec<_> = (0..nums.len())
        .into_iter()
        .map(|i| target[i] - nums[i])
        .collect();

    let detla = chunk_vec(detla);

    let mut result = 0;
    for mut d in detla {
        dbg!(&d);
        if d[0] >= 0 {
            // d.sort();
            // d.into_iter().fold(0, |acc, x| {
            //     result += (x - acc) as i64;
            //     x
            // });
            sum_together(d, &mut result);
        } else {
            // d.sort_by(|a, b| b.partial_cmp(a).unwrap());
            // d.into_iter().fold(0, |acc, x| {
            //     result += -(x - acc) as i64;
            //     x
            // });
            sum_together(d, &mut result);
        };
        //dbg!(&result);
    }
    result
}

fn sum_together(a: Vec<i32>, result: &mut i64) {
    //let flag = if a[0] > 0 { -1 } else { 1 };
    let mut a = vec![a];
    while a.len() != 0 {
        dbg!(&a);

        //*result += a.len() as i64;
        a.iter_mut().for_each(|x| {
            let pn = pick_num(x);
            dbg!(&pn);
            *result += pn.abs() as i64;
            x.iter_mut().for_each(|xx| *xx -= pn);
        });

        a = a
            .into_iter()
            .map(|x| split_zero(x))
            .flatten()
            .filter(|x| x.len() != 0)
            .collect();
    }
}

fn split_zero(l: Vec<i32>) -> Vec<Vec<i32>> {
    l.split(|num| *num == 0)
        .into_iter()
        .map(|s| s.to_vec())
        .collect()
}

fn pick_num(l: &Vec<i32>) -> i32 {
    if l[0] > 0 {
        *l.iter().min().unwrap()
    } else {
        *l.iter().max().unwrap()
    }
}

///////////////////// faster solution below

pub fn minimum_operations(nums: Vec<i32>, target: Vec<i32>) -> i64 {
    let mut prev = 0i32;
    let mut res = 0;
    for (&i, &j) in nums.iter().zip(target.iter()) {
        let diff = j - i;
        if (diff > 0) != (prev > 0) {
            prev = 0;
        }

        println!("{i}, {j}, {diff}, {prev}");

        res += (diff.abs() - prev.abs()).max(0) as i64;
        prev = diff;
    }
    res
}

fn main() {
    //dbg!(chunk_vec(vec![1, 2, 3, 5, -1, -2, 3, -1, -1, 3, 4]));
    //let mut a = vec![-3, -1, -4, -5];
    //a.sort_by(|a, b| b.partial_cmp(a).unwrap());
    //dbg!(a);

    //dbg!(minimum_operations(vec![3, 5, 1, 2], vec![4, 6, 2, 4]));
    //dbg!(minimum_operations(vec![1, 3, 2], vec![2, 1, 4]));

    //dbg!(chunk_vec(vec![2, 0, 1, 6]));
    //dbg!(minimum_operations(vec![5, 9, 2, 2], vec![7, 9, 3, 8]));
    //dbg!(minimum_operations(vec![1, 1, 3, 4], vec![4, 1, 3, 2]));
    // let mut count = 0;
    // sum_together(vec![3, 1, 5, 3, 4, 2], &mut count);
    // dbg!(count);

    //dbg!(pick_num(&vec![-1, -9]));
    dbg!(minimum_operations(
        vec![9, 2, 6, 10, 4, 8, 3, 4, 2, 3],
        vec![9, 5, 5, 1, 7, 9, 8, 7, 6, 5]
    ));
}
