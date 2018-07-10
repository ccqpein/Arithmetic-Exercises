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
#[derive(Debug, Clone, PartialEq)]
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

    fn new_black(v: &i32) -> Node {
        let mut n = Node::new(v);
        n.make_black();
        n
    }
    fn new_red(v: &i32) -> Node {
        let mut n = Node::new(v);
        n.make_red();
        n
    }
    fn make_black(&mut self) {
        self.color = Color::Black;
    }

    fn make_red(&mut self) {
        self.color = Color::Red;
    }

    fn change_color(&mut self) {
        if self.color == Color::Black {
            self.color = Color::Red
        } else {
            self.color = Color::Black
        }
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

    //fix color at ending of insert method
    fn insert(&mut self, v: &i32) {
        if *v == self.enter.val {
            println!("{}", "duplicate value");
            return;
        }

        {
            //need drop target which hold self ownship..
            //we need this ownship to fix color at endding of this method.
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

        //self.fix(v);
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

    fn look_up_father(&mut self, v: &i32) -> (ReTr, &'static str) {
        let mut this: &mut Self = self;
        let mut result: &str = "None";
        let mut next: &str = "None";
        loop {
            if *v == this.enter.val {
                return (Err("no father".to_string()), "this");
            }

            match (this.right.as_ref(), this.left.as_ref()) {
                (Some(r), Some(l)) => {
                    if r.enter.val == *v {
                        result = "right";
                    } else if l.enter.val == *v {
                        result = "left"
                    }
                }
                (Some(r), None) => {
                    if r.enter.val == *v {
                        result = "right";
                    }
                }
                (None, Some(l)) => {
                    if l.enter.val == *v {
                        result = "left";
                    }
                }
                _ => (),
            }

            if *v > this.enter.val {
                if let None = this.right {
                    return (Err("not find".to_string()), "Error");
                }
                next = "right";
            } else {
                if let None = this.left {
                    return (Err("not find".to_string()), "Error");
                }
                next = "left";
            }

            if result != "None" {
                return (Ok(this), result);
            } else if next == "right" {
                this = { this }.right.as_mut().unwrap();
            } else if next == "left" {
                this = { this }.left.as_mut().unwrap();
            }
        }
    }

    //find father, grandfather, and uncle
    fn look_up_family(
        &mut self,
        v: &i32,
    ) -> (
        Result<i32, &'static str>,
        Result<i32, &'static str>,
        Result<i32, &'static str>,
    ) {
        let this: &mut Self = self;
        let grandfather: i32;
        let father: i32;

        match this.look_up_father(v).0 {
            Ok(n) => {
                father = n.enter.val;
            }
            Err(_) => {
                return (
                    Err("father not find"),
                    Err("grandfather not find"),
                    Err("uncle not find"),
                )
            }
        }

        let uncle_option: &Option<Box<Tree>>;
        match this.look_up_father(&father) {
            (Ok(n), s) => {
                grandfather = n.enter.val;
                match s {
                    "left" => uncle_option = &n.right,
                    _ => uncle_option = &n.left,
                };
            }
            (Err(_), _) => {
                return (
                    Ok(father),
                    Err("grandfather not find"),
                    Err("uncle not find"),
                )
            }
        }

        match uncle_option {
            Some(un) => return (Ok(father), Ok(grandfather), Ok(un.enter.val)),
            _ => return (Ok(father), Ok(grandfather), Err("uncle not find")),
        }
    }

    fn cut_left_tree(&self) -> Option<Box<Self>> {
        self.left.clone()
    }

    fn cut_right_tree(&self) -> Option<Box<Self>> {
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

        let right = self.cut_right_tree().unwrap();
        let right_left = right.cut_left_tree().unwrap();
        let right_right = right.cut_right_tree().unwrap();
        new_left.right = Some(right_left);

        self.enter = Node::new_black(&right.enter.val);

        self.right = Some(right_right);

        self.left = Some(Box::new(new_left));
    }

    fn right_shift(&mut self, v: &i32) {
        match self.look_up_father(v).0 {
            Ok(r) => {
                let father = r;
                let mut this = father.cut_right_tree().unwrap();
                let mut son = this.cut_left_tree().unwrap();
                let grand_son = son.cut_right_tree();

                this.left = grand_son;
                son.right = Some(this);
                father.right = Some(son);
                return;
            }
            _ => (),
        }

        let right = self.cut_right_tree();
        let left = self.cut_left_tree().unwrap();
        let left_left = left.cut_left_tree().unwrap();
        let left_right = left.cut_right_tree().unwrap();

        let mut new_right = Tree::new(&self.enter.val);
        new_right.left = Some(left_right);
        new_right.right = right;

        self.enter = Node::new_black(&left.enter.val);
        self.left = Some(left_left);
        self.right = Some(Box::new(new_right));
    }

    //this fix method might not very efficiency because look_up_father..
    //always look from top of tree
    fn fix(&mut self, v: &i32) {
        //step 1
        let mut this: i32 = 0;
        match self.look_up_family(v) {
            (Ok(father), Ok(grandfather), Ok(uncle)) => {
                if let Ok(fff) = self.look_up_father(&grandfather).0 {
                    this = fff.enter.val;
                } else {
                    println!("{}", "no great-grandfather");
                    return;
                }

                if let Ok(ff) = self.look_up(&father) {
                    ff.enter.make_black();
                }

                if let Ok(un) = self.look_up(&uncle) {
                    un.enter.make_black();
                }

                if let Ok(ff) = self.look_up(&grandfather) {
                    ff.enter.make_red();
                }
            }
            (a, b, c) => println!("{:#?}, {:#?}, {:#?}", a, b, c),
        }

        //step 2
        self.left_shift(&this);
        this = self
            .look_up(&{ this })
            .unwrap()
            .left
            .as_mut()
            .unwrap()
            .enter
            .val;

        //stap3
        match self.look_up_family(&this) {
            (Ok(father), Ok(grandfather), _) => {
                if let Ok(ff) = self.look_up(&father) {
                    ff.enter.make_black();
                }

                if let Ok(gf) = self.look_up(&grandfather) {
                    gf.enter.make_red();
                    this = gf.enter.val;
                }
            }
            _ => (),
        }

        this = self.look_up_father(&this).0.unwrap().enter.val;
        self.right_shift(&this);
        self.enter.make_black();
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
