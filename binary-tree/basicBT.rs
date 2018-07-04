use std::fmt;

#[derive(Debug, Clone)]
struct Tree {
    enter: Node,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

/*impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(this: {:#?}, \nleft: {:#?}, \nright: {:#?})",
            self.enter, self.left, self.right
        )
    }
}*/

//for red-black tree
#[derive(Debug, Clone)]
enum Color {
    Black,
    Red,
    White,
}

#[derive(Debug, Clone)]
struct Node {
    val: i32,
    color: Color,
}

impl Node {
    fn new(v: &i32) -> Node {
        Node {
            val: *v,
            color: Color::White,
        }
    }

    fn make_black(&mut self) {
        self.color = Color::Black;
    }

    fn make_red(&mut self) {
        self.color = Color::Red;
    }
}

type ReTr<'a> = Result<&'a mut Tree, String>;

impl Tree {
    fn new(v: &i32) -> Tree {
        let mut root = Node::new(v);
        root.make_black();
        Tree {
            enter: root,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, v: &i32) {
        let target = if *v > self.enter.val {
            &mut self.right
        } else {
            &mut self.left
        };

        match target {
            Some(next) => next.insert(v),
            None => {
                let mut new_node = Node::new(v);
                new_node.make_red();
                let new_tree = Tree {
                    enter: new_node,
                    left: None,
                    right: None,
                };
                *target = Some(Box::new(new_tree));
            }
        }
    }

    fn look_up_recursive(&mut self, v: &i32) -> ReTr {
        if *v == self.enter.val {
            return Ok(self);
        } else if *v > self.enter.val {
            if let Some(_) = self.right {
                let next = self.right.as_mut().unwrap();
                return next.look_up(v);
            } else {
                Err("not find".to_string())
            }
        } else {
            if let Some(_) = self.left {
                let next = self.left.as_mut().unwrap();
                return next.look_up(v);
            } else {
                Err("not find".to_string())
            }
        }
    }

    fn look_up(&mut self, v: &i32) -> ReTr {
        let mut this: &mut Self = self;
        loop {
            if *v == this.enter.val {
                return Ok(this);
            } else if *v > this.enter.val {
                if let None = this.right {
                    return Err("not find".to_string());
                }
                this = { this }.right.as_mut().unwrap();
            } else {
                if let None = this.left {
                    return Err("not find".to_string());
                }
                this = { this }.left.as_mut().unwrap();
            }
        }
    }

    fn look_up_father(&mut self, v: &i32) -> (ReTr, String) {
        let mut this: &mut Self = self;
        loop {
            if *v == this.enter.val {
                return (Err("no father".to_string()), "this".to_string());
            } else if *v == this.right.as_ref().unwrap().enter.val {
                return (Ok(this), "right".to_string());
            } else if *v == this.left.as_ref().unwrap().enter.val {
                return (Ok(this), "left".to_string());
            } else if *v > this.enter.val {
                if let None = this.right {
                    return (Err("not find".to_string()), "Error".to_string());
                }
                this = { this }.right.as_mut().unwrap();
            } else {
                if let None = this.left {
                    return (Err("not find".to_string()), "Error".to_string());
                }
                this = { this }.left.as_mut().unwrap();
            }
        }
    }

    fn cut_left_tree(&mut self) -> Option<Box<Self>> {
        self.left.clone()
    }

    fn cut_right_tree(&mut self) -> Option<Box<Self>> {
        self.right.clone()
    }

    fn left_shift(&mut self, v: &i32) {
        match self.look_up_father(v).0 {
            Ok(r) => {
                let father = r;
                let mut this = father.cut_left_tree().unwrap();
                let mut son = this.cut_right_tree().unwrap();
                let grand_son = son.cut_left_tree().unwrap();

                this.right = Some(grand_son);
                son.left = Some(this);
                father.left = Some(son);
                return;
            }
            _ => (),
        }

        let left = self.cut_left_tree().unwrap();
        let mut new_left = Tree::new(&self.enter.val);
        new_left.left = Some(left);

        let mut right = self.cut_right_tree().unwrap();
        let right_left = right.cut_left_tree().unwrap();
        let right_right = right.cut_right_tree().unwrap();
        new_left.right = Some(right_left);

        self.enter = Node::new(&right.enter.val);

        self.right = Some(right_right);

        self.left = Some(Box::new(new_left));
    }

    fn right_shift(&mut self, v: &i32) {
        let father = self.look_up_father(v).0.unwrap();
        let mut this = father.cut_right_tree().unwrap();
        let mut son = this.cut_left_tree().unwrap();
        let grand_son = son.cut_right_tree().unwrap();

        this.left = Some(grand_son);
        son.right = Some(this);
        father.right = Some(son);
    }
}

fn main() {
    //let mut a = Tree::new(&3);
    //a.insert(&2);
    //a.insert(&1);
    //a.insert(&4);
    //a.insert(&5);

    //println!("{:?}", a);
    //println!("{:?}", a.look_up(&1));
    //println!("{:?}", a.look_up_father(&4));
    //println!("{:?}", a);

    let mut b = Tree::new(&10);
    b.insert(&2);
    b.insert(&1);
    b.insert(&5);
    b.insert(&3);
    b.insert(&6);
    b.insert(&11);

    //println!("{:?}", b);
    //println!("{:?}", b.look_up_father(&2));
    let mut c = b.cut_left_tree().unwrap();
    //println!("{:#?}", c);
    c.left_shift(&2);
    println!("{:#?}", c);

    //let mut c = Tree::new(&2);
    //c.insert(&8);
    //c.insert(&1);
    //c.insert(&5);
    //c.insert(&4);
    //c.insert(&7);
    //c.insert(&9);
    //println!("{:?}", c.look_up_father(&8));
    //c.right_shift(&8);
    //println!("{:#?}", c);
}
