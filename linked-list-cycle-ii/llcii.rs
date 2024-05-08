use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet},
    rc::Rc,
};

struct List {
    V: i32,
    N: Option<Rc<RefCell<List>>>,
}

impl List {
    fn n(&self) -> Option<Rc<RefCell<List>>> {
        Some(self.N.as_ref().unwrap().clone())
    }

    fn view(&self) -> i32 {
        self.V
    }
}

fn detect_cycle(mut head: Rc<RefCell<List>>) -> Option<Rc<RefCell<List>>> {
    let mut table: HashSet<*mut List> = HashSet::new();

    loop {
        //dbg!(&head.borrow().view());
        //dbg!(&RefCell::as_ptr(&head));
        if table.contains(&RefCell::as_ptr(&head)) {
            //dbg!(&RefCell::as_ptr(&head));
            return Some(head);
        }

        table.insert(RefCell::as_ptr(&head));

        if head.as_ref().borrow().n().is_none() {
            return head.as_ref().borrow().n();
        }

        let a = head.as_ref().borrow().n().unwrap().clone();
        head = a;
    }
}

fn main() {
    let mut a = Rc::new(RefCell::new(List { V: 4, N: None }));
    let mut b = Rc::new(RefCell::new(List {
        V: 0,
        N: Some(a.clone()),
    }));
    let mut c = Rc::new(RefCell::new(List {
        V: 2,
        N: Some(b.clone()),
    }));
    let mut d = Rc::new(RefCell::new(List {
        V: 3,
        N: Some(c.clone()),
    }));

    a.as_ref().borrow_mut().N = Some(c.clone());
    //dbg!(detect_cycle(d).unwrap().borrow().view());
    assert_eq!(detect_cycle(d).unwrap().as_ptr(), c.as_ptr());

    /////////////

    let mut a = Rc::new(RefCell::new(List { V: 1, N: None }));
    let mut b = Rc::new(RefCell::new(List {
        V: 2,
        N: Some(a.clone()),
    }));

    a.as_ref().borrow_mut().N = Some(b.clone());

    assert_eq!(detect_cycle(a.clone()).unwrap().as_ptr(), a.as_ptr());
}
