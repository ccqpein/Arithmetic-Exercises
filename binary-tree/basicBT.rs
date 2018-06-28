#[derive(Debug)]
struct Tree {
    enter: Node,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

//for red-black tree
#[derive(Debug)]
enum Color {
    Black,
    Red,
    White,
}

#[derive(Debug)]
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

    fn look_up(&mut self, v: &i32) -> Result<&mut Self, String> {
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

    fn look_up_while(&mut self, v: &i32) -> Result<&mut Self, String> {
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
}

fn main() {
    let mut a = Tree::new(&3);
    a.insert(&2);
    a.insert(&1);

    //println!("{:?}", a);
    //println!("{:?}", a.look_up(&1));
    println!("{:?}", a.look_up_while(&1));
}
