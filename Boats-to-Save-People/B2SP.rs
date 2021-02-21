// pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
//     //use std::collections::VecDeque;
//     people.sort();
//     //let mut pp = people.iter().collect::<VecDeque<_>>();
//     let mut count = 0;
//     let mut inner_limit = limit;
//     for ind in 0..people.len() {
//         if people[ind] <= inner_limit {
//             inner_limit -= people[ind]
//         } else {
//             count += 1;
//             inner_limit = limit - people[ind]
//         }
//     }
//     count + 1
// }

// I foget it can only take 2 people
// pub fn num_rescue_boats_2(mut people: Vec<i32>, limit: i32) -> i32 {
//     people.sort();
//     let mut count = 0;
//     let mut inner_limit = limit;

//     let (mut smallp, mut largep) = (0, people.len() - 1);
//     while smallp <= largep {
//         //println!("{:?}", (smallp, largep));
//         if inner_limit >= people[largep] {
//             inner_limit -= people[largep];
//             if largep == 0 {
//                 break;
//             }
//             largep -= 1;
//             continue;
//         }

//         if inner_limit >= people[smallp] {
//             inner_limit -= people[smallp];
//             smallp += 1;
//             continue;
//         }

//         inner_limit = limit;
//         count += 1
//     }

//     count + 1
// }

pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort();
    let mut count = 0;
    let mut inner_limit = limit;

    let (mut smallp, mut largep) = (0, people.len() - 1);
    while smallp <= largep {
        //println!("{:?}", (smallp, largep));
        if inner_limit >= people[largep] {
            inner_limit -= people[largep];
            if largep == 0 {
                count += 1;
                break;
            }
            largep -= 1;
        }

        if inner_limit >= people[smallp] {
            //inner_limit -= people[smallp];
            smallp += 1;
        }

        inner_limit = limit;
        count += 1
    }

    //count + 1
    count
}

pub fn num_rescue_boats_3(mut people: Vec<i32>, limit: i32) -> i32 {
    use std::collections::VecDeque;
    people.sort();
    let mut pp = people.iter().collect::<VecDeque<_>>();
    let mut count = 0;
    let mut inner_limit = limit;

    while pp.len() > 1 {
        inner_limit -= pp.pop_back().unwrap();
        if *pp[0] <= inner_limit {
            pp.pop_front();
        }
        count += 1;
        inner_limit = limit;
    }

    if pp.len() == 1 {
        count + 1
    } else {
        count
    }
}

fn main() {
    assert_eq!(num_rescue_boats(vec![1, 2], 3), 1);
    assert_eq!(num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
    assert_eq!(num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
    assert_eq!(num_rescue_boats(vec![5, 1, 4, 2], 6), 2);
    assert_eq!(num_rescue_boats(vec![3, 2, 3, 2, 2], 6), 3);

    assert_eq!(num_rescue_boats_3(vec![1, 2], 3), 1);
    assert_eq!(num_rescue_boats_3(vec![3, 2, 2, 1], 3), 3);
    assert_eq!(num_rescue_boats_3(vec![3, 5, 3, 4], 5), 4);
    assert_eq!(num_rescue_boats_3(vec![5, 1, 4, 2], 6), 2);
    assert_eq!(num_rescue_boats_3(vec![3, 2, 3, 2, 2], 6), 3);
}
