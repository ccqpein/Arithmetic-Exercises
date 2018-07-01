#[derive(Debug, Clone)]
struct Tree {
    enter: Node,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

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
}

type ReTr<'a> = Result<&'a mut Tree, String>;

impl Tree {
    fn new(v: &i32) -> Tree {
        Tree {
            enter: Node::new(v),
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
        //println!("target is -> {:?}", target);
        match target {
            Some(next) => {
                //println!("next is -> {:?}", &next);
                next.insert(v)
            }
            None => {
                let new_node = Node::new(v);
                let new_tree = Tree {
                    enter: new_node,
                    left: None,
                    right: None,
                };
                *target = Some(Box::new(new_tree));
            }
        }
    }

    fn look_up_recursive(&mut self, v: &i32) -> Result<&mut Self, String> {
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
            if *v == this.right.as_ref().unwrap().enter.val {
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

    fn cut_left_tree(&mut self) -> Box<Self> {
        self.left.clone().unwrap()
    }

    fn cut_right_tree(&mut self) -> Box<Self> {
        self.right.clone().unwrap()
    }

    fn left_shift(&mut self, v: &i32) {
        let father = self.look_up_father(v).0.unwrap();
        let mut this = father.cut_left_tree();
        let mut son = this.cut_right_tree();
        let grand_son = son.cut_left_tree();

        this.right = Some(grand_son);
        son.left = Some(this);
        father.left = Some(son);
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
    b.left_shift(&2);
    println!("{:?}", b);
}
