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

fn cons(a: Value, b: Value) -> Cons {
    Cons {
        car: Arc::new(a),
        cdr: Arc::new(b),
    }
}
/////////////

impl Cons {
    fn flatten(self) -> Vec<Value> {
        let mut result = vec![];
        match Arc::try_unwrap(self.car).unwrap() {
            Value::List(l) => {
                result.append(&mut l.flatten());
                match Arc::try_unwrap(self.cdr).unwrap() {
                    e @ Value::Integer(_) => result.push(e),
                    Value::List(l) => result.append(&mut l.flatten()),
                    Value::Nil => return result,
                }
            }
            Value::Integer(v) => {
                result.push(Value::Integer(v));
                match Arc::try_unwrap(self.cdr).unwrap() {
                    e @ Value::Integer(_) => result.push(e),
                    Value::List(l) => result.append(&mut l.flatten()),
                    Value::Nil => return result,
                }
            }
            _ => {}
        }
        result
    }
}

fn main() {
    let test0 = cons(
        Value::Integer(1),
        Value::List(cons(
            Value::List(cons(
                Value::Integer(3),
                Value::List(cons(Value::Integer(2), Value::Nil)),
            )),
            Value::List(cons(
                Value::List(cons(Value::Integer(5), Value::Nil)),
                Value::Nil,
            )),
        )),
    );

    dbg!(test0.flatten());
}
