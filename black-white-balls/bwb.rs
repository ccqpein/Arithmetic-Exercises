#[derive(Clone, PartialEq, Eq, Debug)]
enum BallColor {
    Black,
    White,
}

type BallChain = Vec<BallColor>;

fn concat_white(last: &BallChain) -> Option<BallChain> {
    let mut result = last.clone();
    result.push(BallColor::White);
    Some(result)
}

fn concat_black(last: &BallChain) -> Option<BallChain> {
    let mut result = last.clone();
    if let Some(BallColor::Black) = result.last() {
        return None;
    } else {
        result.push(BallColor::Black);
        Some(result)
    }
}

// ball_pool is (white_left, black_left)
fn get_next(
    last: BallChain,
    (white_left, black_left): (usize, usize),
) -> Vec<(BallChain, (usize, usize))> {
    let mut result = vec![];
    if white_left > 0 {
        if let Some(c) = concat_white(&last) {
            result.push((c, (white_left - 1, black_left)))
        }
    }

    if black_left > 0 {
        if let Some(c) = concat_black(&last) {
            result.push((c, (white_left, black_left - 1)))
        }
    }

    result
}

// this one cannot filter the circle like 100, 010, 001, they are actully same
fn start((white, black): (usize, usize)) -> Vec<Vec<BallColor>> {
    let mut bucket = get_next(vec![], (white, black));
    //dbg!(&bucket);

    let mut temp = vec![];
    // loop time
    for _ in 0..(white + black - 1) {
        for (last, ball_left) in bucket {
            temp.append(&mut get_next(last, ball_left))
        }
        bucket = temp;
        temp = vec![];
    }

    //dbg!(&bucket);
    //let mut count = bucket.len();
    bucket
        .into_iter()
        .filter(|(chain, _)| {
            if chain.first().unwrap() == chain.last().unwrap()
                && *chain.first().unwrap() == BallColor::Black
                && chain.len() != 1
            {
                false
            } else {
                true
            }
        })
        .map(|(chain, _)| chain)
        .collect()
}

fn main() {
    dbg!(start((1, 2))); //=> []
    dbg!(start((0, 1))); //=> [[Black]]
    dbg!(start((2, 1))); //=> [[001],[010],[100]]
    dbg!(start((0, 2))); //=> []
}
