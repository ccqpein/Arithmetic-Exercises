use std::rc::Rc;

#[derive(Debug)]
struct DLList<T> {
    Node: T,
    Prev: Option<Rc<DLList<T>>>,
    Next: Option<Rc<DLList<T>>>,
}

impl<T: Copy> DLList<T> {
    fn new(val: T) -> Self {
        DLList {
            Node: val,
            Prev: None,
            Next: None,
        }
    }

    fn cons(val: T, li: &mut Rc<Self>) -> Rc<Self> {
        //let cloner = Rc::clone(li);
        let result = Rc::new(DLList {
            Node: val,
            Next: None,
            Prev: None,
        });

        //let cloner = Rc::into_raw(Rc::clone(&result));
        (*Rc::get_mut(li).unwrap()).Prev = Some(Rc::clone(&result));
        //(*Rc::get_mut((*Rc::get_mut(li).unwrap()).Prev.as_mut().unwrap()).unwrap()).Next =
        //Some(Rc::clone(li));

        /*unsafe {
            *cloner = DLList {
                Node: val,
                Next: Some(Rc::clone(li)),
                Prev: None,
            };
        }*/

        return result;
    }
}

//check type
fn fuck(a: ()) {}

fn main() {
    let mut a = Rc::new(DLList::new(1));
    let mut b = DLList::cons(2, &mut a);
    //let b = DLList::new(2);

    println!("a prev {:?}", a.Prev);
    //println!("b next {:?}", &b.Next);

    //let temp = Rc::clone(&a);
    println!("test a {:#?}", Rc::get_mut(&mut a));
    println!(
        "test prev {:#?}",
        (*Rc::get_mut(&mut a).unwrap()).Prev.as_mut()
    );

    println!(
        "test2 : {:?}",
        (*Rc::get_mut(&mut a).unwrap()).Prev.as_mut().unwrap()
    );

    //fuck(Rc::get_mut(Rc::get_mut(&mut b).unwrap().Next.as_mut().unwrap()).unwrap());

    //line below cannot work because get_mut() method work only in "no other Rc or Weak pointers to the same inner value"
    //value b (value result from cons()) actually be cloned once in cons(), there is what "other Rc" happened.
    (*Rc::get_mut(&mut b).unwrap()).Next = Some(Rc::new(DLList::new(4)));

    //(*b).Next = Some(Rc::clone(&a));
    println!("done clone");

    println!("a : {:?}", a);
    println!("b : {:?}", b);
}
