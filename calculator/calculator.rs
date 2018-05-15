enum MathExp {
    Value(i32),
    Add(Box<MathExp>, Box<MathExp>),
    Minus(Box<MathExp>, Box<MathExp>),
    Multiply(Box<MathExp>, Box<MathExp>),
    Divide(Box<MathExp>, Box<MathExp>),
}

impl MathExp {
    fn evaluate(&mut self) -> i32 {
        match *self {
            MathExp::Value(a) => return a,
            MathExp::Add(a, b) => Box::into_raw(a).evaluate() + Box::into_raw(b).evaluate(),
            MathExp::Minus(a, b) => Box::into_raw(a).evaluate() - Box::into_raw(b).evaluate(),
            MathExp::Multiply(a, b) => Box::into_raw(a).evaluate() * Box::into_raw(b).evaluate(),
            MathExp::Divide(a, b) => Box::into_raw(a).evaluate() / Box::into_raw(b).evaluate(),
        }
    }
}

fn main() {}
