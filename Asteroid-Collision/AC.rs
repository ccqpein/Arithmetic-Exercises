pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut asteroid_inner = asteroids.iter();
    let mut stack: Vec<i32> = vec![*asteroid_inner.next().unwrap()];

    while let Some(v) = asteroid_inner.next() {
        loop {
            if stack.last().is_none() {
                stack.push(*v);
                break;
            }
            match (stack.last().unwrap().is_positive(), v.is_positive()) {
                (true, false) => {
                    //-> <-
                    if *stack.last().unwrap() > v.abs() {
                        break;
                    } else if *stack.last().unwrap() == v.abs() {
                        stack.pop();
                        break;
                    } else {
                        stack.pop();
                    }
                }
                _ => {
                    stack.push(*v);
                    break;
                }
            }
        }
    }

    stack
}

fn main() {
    println!("{:?}", asteroid_collision(vec![5, 10, -5])); //=> [5,10]
    println!("{:?}", asteroid_collision(vec![8, -8])); //=> []
    println!("{:?}", asteroid_collision(vec![10, 2, -5])); //=> [10]
    println!("{:?}", asteroid_collision(vec![1, -2, -2, -2])); //=> [-2,-2,-2]
}
