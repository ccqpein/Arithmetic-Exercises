use anyhow::Result;
use std::{cell::RefCell, collections::BTreeMap, fmt::Display, rc::Rc};

#[derive(Debug)]
struct LinkList {
    key: i64,
    value: i64,

    prev: Option<Rc<RefCell<LinkList>>>,
    next: Option<Rc<RefCell<LinkList>>>,
}

// impl Display for LinkList {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}:{}", self.key, self.value)
//     }
// }

impl LinkList {
    fn show(&self) -> String {
        format!("{}:{}", self.key, self.value,)
    }
}

#[derive(Debug)]
struct LRUCache {
    cap: usize,
    inner_map: BTreeMap<i64, Rc<RefCell<LinkList>>>,

    first: Option<Rc<RefCell<LinkList>>>,
    last: Option<Rc<RefCell<LinkList>>>,
}

impl LRUCache {
    fn new(cap: usize) -> Self {
        Self {
            cap,
            inner_map: BTreeMap::new(),
            first: None,
            last: None,
        }
    }

    fn pop_end(&mut self) -> Result<()> {
        let last = self
            .last
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("empty cache, not last"))?
            .clone();

        let llast = match last.borrow().prev.as_ref() {
            Some(x) => x.clone(),
            None => {
                self.first = None;
                self.last = None;
                self.inner_map
                    .remove(&last.borrow().key)
                    .ok_or_else(|| anyhow::anyhow!("inner map doesnt have this key"))?;
                return Ok(());
            }
        };

        self.inner_map
            .remove(&last.borrow().key)
            .ok_or_else(|| anyhow::anyhow!("inner map doesnt have this key"))?;
        llast.borrow_mut().next = None;
        self.last = Some(llast);

        Ok(())
    }

    fn get(&mut self, k: &i64) -> Result<Option<i64>> {
        let l = match self.inner_map.get(k) {
            Some(ll) => ll.clone(),
            None => return Ok(None),
        };

        let res = l.borrow().value;

        self.remove(k)?;

        //println!("in get {}: ", self.inner_map.len());
        //self.show_me_the_map();

        self.add(*k, res)?;

        Ok(Some(res))
    }

    fn remove(&mut self, k: &i64) -> Result<()> {
        let l = match self.inner_map.get(k) {
            Some(ll) => ll.clone(),
            None => anyhow::bail!("wrong key {} to remove", *k),
        };

        // if it is the last
        if l.borrow().next.is_none() {
            self.pop_end()?;
            return Ok(());
        }

        // if it is the first
        if l.borrow().prev.is_none() {
            self.first = l.borrow().next.clone();
            self.inner_map.remove(k);

            match self.first.as_ref(){
                Some(next) => next.borrow_mut().prev = None,
                None => anyhow::bail!("it is the only one element in list. it should be checked as the last logic upper"),
            }
            return Ok(());
        }

        match (l.borrow().prev.as_ref(), l.borrow().next.as_ref()) {
            (Some(p), Some(n)) => {
                p.borrow_mut().next = Some(n.clone());
                n.borrow_mut().prev = Some(p.clone());
            }
            _ => anyhow::bail!("screw up too much"),
        }

        self.inner_map.remove(k);

        Ok(())
    }

    fn add(&mut self, k: i64, v: i64) -> Result<()> {
        // check cap
        if self.cap == self.inner_map.len() && !self.inner_map.get(&k).is_some() {
            // pop end
            self.pop_end()?
        }

        if self.inner_map.get(&k).is_some() {
            self.remove(&k)?;
        }

        // add the new to head
        let l = LinkList {
            key: k,
            value: v,
            prev: None,
            next: self.first.clone(),
        };

        let l = Rc::new(RefCell::new(l));
        self.inner_map.insert(k, l.clone());

        match &self.first {
            Some(f) => f.borrow_mut().prev = Some(l.clone()),
            None => (),
        }
        self.first = Some(l.clone());

        if self.inner_map.len() == 1 {
            self.last = Some(l);
        }

        Ok(())
    }

    fn show_me_the_list(&self) -> Vec<(i64, i64)> {
        let mut this = self.first.clone();

        let mut res = vec![];

        loop {
            if this.is_none() {
                break;
            }

            res.push((
                this.as_ref().unwrap().borrow().key,
                this.as_ref().unwrap().borrow().value,
            ));

            let xx = this.as_ref().unwrap().borrow().next.clone();
            this = xx;
        }

        res
    }

    fn show_me_the_map(&self) {
        let mut this = self.first.clone();

        println!(
            "lc first is {:?}, last is {:?}, len is {}",
            if self.first.as_ref().is_some() {
                Some(self.first.as_ref().unwrap().borrow().show())
            } else {
                None
            },
            if self.last.as_ref().is_some() {
                Some(self.last.as_ref().unwrap().borrow().show())
            } else {
                None
            },
            self.inner_map.len(),
        );
        loop {
            if this.is_none() {
                break;
            }

            println!("this: {}", this.as_ref().unwrap().borrow().show());
            println!(
                "prev: {:?}",
                if this.as_ref().unwrap().borrow().prev.is_some() {
                    Some(
                        this.as_ref()
                            .unwrap()
                            .borrow()
                            .prev
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .show(),
                    )
                } else {
                    None
                }
            );
            println!(
                "next: {:?}",
                if this.as_ref().unwrap().borrow().next.is_some() {
                    Some(
                        this.as_ref()
                            .unwrap()
                            .borrow()
                            .next
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .show(),
                    )
                } else {
                    None
                }
            );

            let xx = this.as_ref().unwrap().borrow().next.clone();
            this = xx;
        }
    }

    fn check_all_rc_count(&self) -> Vec<usize> {
        let mut this = self.first.clone();

        let mut res = vec![];

        loop {
            if this.is_none() {
                break;
            }

            res.push(Rc::strong_count(this.as_ref().unwrap()));

            let xx = this.as_ref().unwrap().borrow().next.clone();
            this = xx;
        }

        res
    }
}

fn main() {
    let mut lc = LRUCache::new(3);
    lc.add(1, 1);
    //println!("{:?}", lc);
    println!("after add 1:1");
    lc.show_me_the_map();
    println!("all rc count: {:?}", lc.check_all_rc_count());

    lc.add(2, 2);
    //println!("{:?}", lc.inner_map);
    println!("\nafter add 2:2");
    lc.show_me_the_map();
    println!("all rc count: {:?}", lc.check_all_rc_count());

    lc.add(3, 3);
    println!("\nafter add 3:3");
    lc.show_me_the_map();

    lc.add(3, 3);
    println!("\nafter add 3:3");
    lc.show_me_the_map();

    lc.add(2, 2);
    println!("\nafter add 2:2");
    lc.show_me_the_map();
    println!("map len: {}", lc.inner_map.len());

    lc.add(4, 4);
    println!("\nafter add 4:4");
    lc.show_me_the_map();

    //:= test get
    assert_eq!(lc.get(&3).unwrap(), Some(3));
    println!("\nafter get 3:3");
    lc.show_me_the_map(); // change to 3 4 2

    assert_eq!(lc.get(&100).unwrap(), None);
    println!("\nafter get 100");
    lc.show_me_the_map(); // change to 3 4 2

    // test the rc count
    println!("all rc count: {:?}", lc.check_all_rc_count());

    //println!("{:?}", lc.inner_map);
    println!("{:?}", lc.show_me_the_list());
}
