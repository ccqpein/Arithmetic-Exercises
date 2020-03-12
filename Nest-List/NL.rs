/// copy from ../cons
use std::sync::Arc;

#[derive(Debug)]
enum Value {
    Nil,
    Integer(i64),
    List(Cons),
}

#[derive(Debug)]
struct Cons {
    car: Arc<Value>,
    cdr: Arc<Value>,
}

fn cons(a: Value, b: Cons) -> Cons {
    Cons {
        car: Arc::new(a),
        cdr: Arc::new(Value::List(b)),
    }
}
/////////////

impl Cons {
    fn flatten(self) -> Vec<Value> {
        let mut result = vec![];
        match Arc::try_unwrap(self.car).unwrap() {
            Value::List(l) => result.append(&mut l.flatten()),
            Value::Integer(v) => result.push(Value::Integer(v)),
            _ => {}
        }
        result
    }
}

fn main() {
    let test0 = cons(
        Value::Integer(1),
        cons(
            cons(Value::Integer(3), cons(Value::Integer(2), Value::Nil)),
            cons(cons(Value::Integer(5), Value::Nil), Value::Nil),
        ),
    );

    dbg!(test0.flatten());
}
