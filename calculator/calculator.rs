enum Ops {
    Val(i32),
    Add,
    Minus,
    Mul,
    Div,
}
/*
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
}*/

trait Exp {
    fn evaluate(&self) -> i32;
}

struct None();

impl Exp for None {
    fn evaluate(&self) -> i32 {
        return 0;
    }
}

struct Expression {
    ops: Ops,
    left: Box<Exp>,
    right: Box<Exp>,
}

impl Exp for Expression {
    fn evaluate(&self) -> i32 {
        match self.ops {
            Ops::Val(a) => return a,
            Ops::Add => return self.left.evaluate() + self.right.evaluate(),
            Ops::Minus => return self.left.evaluate() - self.right.evaluate(),
            Ops::Mul => return self.left.evaluate() * self.right.evaluate(),
            Ops::Div => return self.left.evaluate() / self.right.evaluate(),
        }
    }
}

//1+2*3
fn main() {
    use Ops::*;
    let test = Expression {
        ops: Add,
        left: Box::new(Expression {
            ops: Val(1),
            left: Box::new(None()),
            right: Box::new(None()),
        }),
        right: Box::new(Expression {
            ops: Mul,
            left: Box::new(Expression {
                ops: Val(2),
                left: Box::new(None()),
                right: Box::new(None()),
            }),
            right: Box::new(Expression {
                ops: Val(3),
                left: Box::new(None()),
                right: Box::new(None()),
            }),
        }),
    };

    println!("{:?}", test.evaluate());
}
