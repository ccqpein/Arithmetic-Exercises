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

fn cons(a: Value, b: Value) -> Value {
    Value::List(Cons {
        car: Arc::new(a),
        cdr: Arc::new(b),
    })
}

fn main() {
    let a = cons(Value::Integer(1), cons(Value::Integer(2), Value::Nil));
    println!("{:?}", a);
}
