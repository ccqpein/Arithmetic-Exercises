#[derive(Debug, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl Default for ListNode {
    fn default() -> ListNode {
        ListNode { val: 0, next: None }
    }
}

impl ListNode {
    fn new(i: i32) -> Self {
        ListNode { val: i, next: None }
    }

    //:= https://stackoverflow.com/questions/50957738/how-to-copy-a-raw-pointer-in-rust?noredirect=1#comment88913158_50957738
    /*
    fn add_l(&mut self, l: &Vec<i32>) {
        //let mut p: *mut ListNode = self as *mut ListNode;
        let org = self;
        let mut p: *mut ListNode = org as *mut ListNode;

        for i in l {
            unsafe {
                //println!("{:?}", *p);
                (*p).next = Some(Box::new(ListNode::new(*i)));
                let temp_b = Box::from_raw(p);
                p = Box::into_raw(temp_b.next.unwrap());
            };
        }
    }*/

    //https://stackoverflow.com/questions/50957738/how-to-copy-a-raw-pointer-when-implementing-a-linked-list-in-rust/50970414#50970414
    fn add_l(&mut self, l: &[i32]) {
        let mut head = self;

        for &val in l {
            head.next = Some(Box::new(ListNode::new(val)));
            head = { head }.next.as_mut().unwrap();
        }
    }

    fn add(&mut self, i: &i32) {
        self.next = Some(Box::new(ListNode::new(*i)));
    }

    fn add_head(self, i: i32) -> Self {
        ListNode {
            val: i,
            next: Some(Box::new(self)),
        }
    }

    fn new_l(l: Vec<i32>) -> Self {
        let mut inner_l = l;
        let mut temp = Self::new(*&inner_l.pop().unwrap());
        for i in inner_l.into_iter().rev() {
            temp = temp.add_head(i)
        }
        temp
    }

    fn removeNthFromEnd(&mut self, n: i32) {
        if let None = self.next {
            println!("Error");
            return;
        }

        let mut n_h = 1;
        {
            let mut head_2 = self.next.as_ref().unwrap();

            for _ in 1..n {
                head_2 = { head_2 }.next.as_ref().unwrap_or(head_2);
            }

            while let Some(_) = { &head_2 }.next {
                n_h += 1;
                head_2 = { head_2 }.next.as_ref().unwrap();
            }
        }
        println!("number is {}", n_h);

        let mut temp: Option<Box<ListNode>>;
        {
            let mut head_2 = self.next.as_mut().unwrap();
            for _ in 1..n_h {
                head_2 = { head_2 }.next.as_mut().unwrap();
            }
            temp = head_2.next.clone()
        }

        let mut head_2 = self;

        for _ in 1..n_h {
            head_2 = { head_2 }.next.as_mut().unwrap();
        }
        head_2.next = temp;
    }
}

fn main() {
    //Known bug: cannot delete first element.
    //Of course cannot move list which own only one element (will panic)

    let mut a = ListNode::new_l(vec![1, 2, 3, 4, 5]);
    a.removeNthFromEnd(2);
    println!("{:?}", a);

    let mut b = ListNode::new_l(vec![1, 2]);
    b.removeNthFromEnd(1);
    println!("{:?}", b);

    //Failed! Panic
    //let mut c = ListNode::new_l(vec![1]);
    //c.removeNthFromEnd(1);
    //println!("{:?}", c);
}
