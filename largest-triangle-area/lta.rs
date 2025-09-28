// pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
//     let mut max_x: i32 = 0;
//     let mut min_x: i32 = 0;
//     let mut max_y: i32 = 0;
//     let mut min_y: i32 = 0;

//     for p in points {
//         if p[1] >= max_y {
//             max_y = p[1]
//         }

//         if p[1] <= min_y {
//             min_y = p[1]
//         }

//         if p[0] >= max_x {
//             max_x = p[0]
//         }

//         if p[0] <= min_x {
//             min_x = p[0]
//         }
//     }

//     match (min_x, max_x, min_y, max_y) {
//         (0, _, _, _) => return (max_x * (max_y - min_y)) as f64 / 2.0,
//         (_, 0, _, _) => return (min_x.abs() * (max_y - min_y)) as f64 / 2.0,
//         (_, _, 0, _) => return ((max_x - min_x) * max_y) as f64 / 2.0,
//         (_, _, _, 0) => return ((max_x - min_x) * min_y.abs()) as f64 / 2.0,
//         _ => {
//             //
//             let rate1 = (max_y - min_y) / max_y;
//             let new_x1 = (max_x - min_x) * rate1;
//             let area1 = new_x1 as f64 * (max_y - min_y) as f64 / 2.0;

//             //
//             let rate2 = (min_y - max_y) / min_y;
//             let new_x2 = (max_x - min_x) * rate2;
//             let area2 = new_x2 as f64 * (max_y - min_y) as f64 / 2.0;

//             //
//             let rate3 = (max_x - min_x) / max_x;
//             let new_y1 = rate3 * (max_y - min_y);
//             let area3 = new_y1 as f64 * (max_x - min_x) as f64 / 2.0;

//             //
//             let rate4 = (min_x - max_x) / min_x;
//             let new_y2 = rate4 * (max_y - min_y);
//             let area4 = new_y2 as f64 * (max_x - min_x) as f64 / 2.0;

//             return area1.max(area2).max(area3).max(area4);
//         }
//     }
// }

// Area = 1/2 |(x1y2 + x2y3 + x3y1) - (x2y1 + x3y2 + x1y3)
pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let area_cal = |(x0, y0), (x1, y1), (x2, y2)| -> f64 {
        ((x0 * y1 + x1 * y2 + x2 * y0) as f64 - (x1 * y0 + x2 * y1 + x0 * y2) as f64).abs() / 2.0
    };

    let mut area = 0.0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for m in j + 1..points.len() {
                let new_a = area_cal(
                    (points[i][0], points[i][1]),
                    (points[j][0], points[j][1]),
                    (points[m][0], points[m][1]),
                );

                if new_a > area {
                    area = new_a
                }
            }
        }
    }

    area
}

fn main() {}
