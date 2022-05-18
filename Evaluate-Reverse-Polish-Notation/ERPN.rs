pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::with_capacity(tokens.len());
    let mut a;
    let mut b;
    for c in tokens {
        match c.as_str() {
            "+" => {
                b = stack.pop().unwrap();
                a = stack.pop().unwrap();
                stack.push(a + b)
            }
            "-" => {
                b = stack.pop().unwrap();
                a = stack.pop().unwrap();
                stack.push(a - b)
            }
            "*" => {
                b = stack.pop().unwrap();
                a = stack.pop().unwrap();
                stack.push(a * b)
            }

            "/" => {
                b = stack.pop().unwrap();
                a = stack.pop().unwrap();
                stack.push(a / b)
            }
            _ => stack.push(c.parse::<i32>().unwrap()),
        }
    }
    stack.pop().unwrap()
}

fn main() {
    assert_eq!(
        eval_rpn(
            ["2", "1", "+", "3", "*"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        9
    );

    assert_eq!(
        eval_rpn(
            ["4", "13", "5", "/", "+"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        6
    );

    assert_eq!(
        eval_rpn(
            ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        22
    );
}
