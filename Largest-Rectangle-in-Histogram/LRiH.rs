pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
    // one start , one end
    let mut cache: Vec<(usize, usize)> =
        (0..heights.len() + 2).into_iter().map(|d| (d, d)).collect();
    let mut hh = vec![0];
    hh.append(&mut heights);
    hh.push(0);

    for i in 1..hh.len() - 1 {
        for ii in 0..i {
            if hh[ii] < hh[i] {
                cache[i].0 = ii
            }
        }
    }

    for i in (1..hh.len() - 1).rev() {
        for ii in (i..hh.len()).rev() {
            if hh[ii] < hh[i] {
                cache[i].1 = ii
            }
        }
    }

    dbg!(&cache);
    cache
        .into_iter()
        .enumerate()
        .map(|(ind, (a, b))| {
            if b != a {
                if b == ind || a == ind {
                    (b - a) as i32 * hh[ind]
                } else {
                    (b - a - 1) as i32 * hh[ind]
                }
            } else {
                hh[ind]
            }
        })
        .max()
        .unwrap()
}

// right solution, older one is too slow
pub fn largest_rectangle_area2(heights: Vec<i32>) -> i32 {
    heights
        .iter()
        .chain(&[0])
        .enumerate()
        .fold((vec![], 0), |(mut v, mut ans), (i, &x)| {
            println!("\n{:?}", &v);
            println!("{:?}", &ans);
            println!("{},{}", i, x);
            while let Some(&y) = v.last() {
                if x > heights[y] {
                    break;
                }
                let height = heights[v.pop().unwrap()];
                let tmp: i32 = if let Some(&i) = v.last() {
                    i as i32
                } else {
                    -1
                };
                let weight = i as i32 - tmp - 1;
                println!("weight: {weight}, height: {height}");
                ans = ans.max(height * weight);
            }
            v.push(i);
            (v, ans)
        })
        .1
}

fn main() {
    dbg!(largest_rectangle_area2(vec![2, 1, 5, 6, 2, 3]));
    //dbg!(largest_rectangle_area(vec![2, 4]));
    //assert_eq!(4, largest_rectangle_area(vec![2, 3]));
}
