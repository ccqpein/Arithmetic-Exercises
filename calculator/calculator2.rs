enum MathExp {
    Value(i32),
    Add(Box<MathExp>, Box<MathExp>),
    Minus(Box<MathExp>, Box<MathExp>),
    Multiply(Box<MathExp>, Box<MathExp>),
    Divide(Box<MathExp>, Box<MathExp>),
}

impl MathExp {
    fn evaluate(&self) -> i32 {
        match self {
            MathExp::Value(a) => return *a,
            MathExp::Add(a, b) => a.evaluate() + b.evaluate(),
            MathExp::Minus(a, b) => a.evaluate() - b.evaluate(),
            MathExp::Multiply(a, b) => a.evaluate() * b.evaluate(),
            MathExp::Divide(a, b) => a.evaluate() / b.evaluate(),
        }
    }
}

fn main() {
    use MathExp::*;
    //1 + 2 * 3
    let test = Add(
        Box::new(Value(1)),
        Box::new(Multiply(Box::new(Value(2)), Box::new(Value(3)))),
    );

    println!("{:?}", test.evaluate());
}
