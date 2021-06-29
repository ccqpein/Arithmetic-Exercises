use std::{borrow::Borrow, cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Cons {
    val: i32,
    cdr: Option<Rc<RefCell<Cons>>>,
}

impl Cons {
    fn new(val: i32) -> Self {
        Cons { val, cdr: None }
    }

    fn cons(&mut self, val: i32) {
        self.cdr = Some(Rc::new(RefCell::new(Cons::new(val))));
    }

    fn append(&mut self, slice: &[i32]) {
        if slice.len() == 1 {
            self.cons(slice[0]);
            return;
        } else {
            let mut a = Cons::new(slice[0]);
            a.append(&slice[1..]);
            self.cdr = Some(Rc::new(RefCell::new(a)))
        }
    }
}

#[derive(Debug)]
struct Cons1 {
    val: i32,
    last: Option<Rc<RefCell<Cons1>>>,
    cdr: Option<Rc<RefCell<Cons1>>>,
}

impl Cons1 {
    fn new(val: i32) -> Self {
        Cons1 {
            val,
            last: None,
            cdr: None,
        }
    }

    fn cons(a: Rc<RefCell<Self>>, b: i32) -> Rc<RefCell<Self>> {
        let next = Cons1::new(b);
        Self::cons_rc(a, Rc::new(RefCell::new(next)))
    }

    fn cons_rc(a: Rc<RefCell<Self>>, b: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        b.borrow_mut().last = Some(a.clone());
        a.borrow_mut().cdr = Some(b.clone());
        a
    }

    fn append(a: Rc<RefCell<Self>>, slice: &[i32]) -> Rc<RefCell<Self>> {
        if slice.len() == 1 {
            Self::cons(a, slice[0])
        } else {
            let aa = Rc::new(RefCell::new(Self::new(slice[0])));
            Self::cons_rc(a, Self::append(aa, &slice[1..]))
        }
    }
}

fn main() {
    let mut a = Cons::new(0);
    a.append(&vec![1, 2, 3, 4]);
    dbg!("{:?}", a);

    let a = Cons1::new(0);
    let thiscons1 = Cons1::append(Rc::new(RefCell::new(a)), &vec![1, 2, 3, 4]);
    let mut a = thiscons1.clone();
    loop {
        println!(
            "{}, last is_some?: {}",
            RefCell::borrow(&a).val,
            RefCell::borrow(&a).last.is_some()
        );
        if RefCell::borrow(&a).cdr.is_none() {
            break;
        } else {
            let temp = RefCell::borrow(&a).cdr.as_ref().unwrap().clone();
            a = temp;
        }
    }

    let mut a = thiscons1.clone();
    loop {
        println!("this point {:?}", RefCell::as_ptr(&a));
        if let Some(l) = RefCell::borrow(&a).last.as_ref() {
            println!(
                "last v: {:?}, last point: {:?}",
                RefCell::borrow(&l).val,
                RefCell::as_ptr(&l)
            );
        } else {
            println!("None");
        }

        if RefCell::borrow(&a).cdr.is_none() {
            break;
        } else {
            let temp = RefCell::borrow(&a).cdr.as_ref().unwrap().clone();
            a = temp;
        }
    }
}
