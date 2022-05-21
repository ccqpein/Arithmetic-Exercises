// pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
//     use std::collections::VecDeque;
//     let dv: VecDeque = card_points.into();
//     let mut head: Option<i32>;
//     let mut tail: Option<i32>;
//     let mut sum = 0;

//     for _ in 0..k {
//         if head.is_none() {
//             head = dv.pop_front();
//         }
//         if tail.is_none() {
//             tail = dv.pop_back();
//         }

//         sum += i32::max(head.unwrap_or(0), tail.unwrap_or(0));

//         match (dv.pop_front(), dv.pop_back()) {
//             (None, None) => (0, 0),
//             (None, Some(t)) => (0, t),
//             (Some(h), None) => (h, 0),
//             (Some(h), Some(t)) => (h, t),
//         };
//     }
// }

pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    //helper(&card_points, 0, card_points.len() - 1, k)
    //helper2(&card_points, 0, card_points.len() - 1, k)
    helper3(&card_points, k)
}

fn helper(card_points: &[i32], left: usize, right: usize, k: i32) -> i32 {
    if k == 0 {
        return 0;
    }

    if card_points[left] > card_points[right] {
        helper(card_points, left + 1, right, k - 1) + card_points[left]
    } else if card_points[left] < card_points[right] {
        helper(card_points, left, right - 1, k - 1) + card_points[right]
    } else {
        i32::max(
            helper(card_points, left + 1, right, k - 1) + card_points[left],
            helper(card_points, left, right - 1, k - 1) + card_points[right],
        )
    }
}

fn helper2(card_points: &[i32], left: usize, right: usize, k: i32) -> i32 {
    if k == 0 {
        return 0;
    }

    i32::max(
        helper2(card_points, left + 1, right, k - 1) + card_points[left],
        helper2(card_points, left, right - 1, k - 1) + card_points[right],
    )
}

fn helper3(card_points: &[i32], k: i32) -> i32 {
    let length = card_points.len();
    let mut result = card_points[0..k as usize].iter().sum();
    for i in 0..=k as usize {
        if result
            < card_points[0..k as usize - i].iter().sum::<i32>()
                + card_points[length - i..length].iter().sum::<i32>()
        {
            result = card_points[0..k as usize - i].iter().sum::<i32>()
                + card_points[length - i..length].iter().sum::<i32>()
        }
    }
    result
}

fn main() {
    assert_eq!(max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
    assert_eq!(max_score(vec![2, 2, 2], 2), 4);
    assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
    assert_eq!(
        max_score(
            vec![
                53, 14, 91, 35, 51, 9, 80, 27, 6, 15, 77, 86, 34, 62, 55, 45, 91, 45, 23, 75, 66,
                42, 62, 13, 34, 18, 89, 67, 93, 83, 100, 14, 92, 73, 48, 2, 47, 93, 99, 100, 88,
                84, 48
            ],
            43
        ),
        2429
    );
}
