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

        match target {
            Some(next) => next.insert(v),
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
}

fn main() {}
