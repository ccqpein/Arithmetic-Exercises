pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
    let mut all_x: Vec<i32> = points.into_iter().map(|x| x[0]).collect();
    all_x.sort();

    let mut result = 1;
    let init = all_x[0];
    all_x.into_iter().fold(init, |s, x| {
        if x - s > w {
            result += 1;
            x
        } else {
            s
        }
    });

    return result;
}

fn main() {}
