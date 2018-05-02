#[derive(Debug)]
struct ListNode {
    Val: i32,
    Next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(input: Vec<i32>) -> Self {
        let mut result = ListNode {
            Val: input[0],
            Next: None,
        };
        {
            let mut p_result = &mut result;
            for i in input.into_iter().skip(1) {
                p_result.Next = Some(Box::new(ListNode { Val: i, Next: None }));
                p_result = { p_result }.Next.as_mut().unwrap();
            }
        }
        result
    }

    fn middle_node(&self) -> &Self {
        let mut temp0 = self;
        let mut temp1 = if let Some(ref a) = self.Next {
            a
        } else {
            return temp0;
        };

        loop {
            if let None = temp1.Next {
                return temp0.Next.as_ref().unwrap();
            }
            match temp1.Next.as_ref().unwrap().Next {
                Some(_) => {}
                None => return temp0.Next.as_ref().unwrap(),
            }

            temp0 = &temp0.Next.as_ref().unwrap();
            temp1 = &temp1.Next.as_ref().unwrap().Next.as_ref().unwrap();
        }
    }
}

fn main() {
    let a = ListNode::new(vec![1, 2, 3, 4, 5]);
    let b = ListNode::new(vec![1, 2, 3, 4, 5, 6]);
    let c = ListNode::new(vec![1]);

    println!("{:#?}", a.middle_node());
    println!("{:#?}", b.middle_node());
    println!("{:#?}", c.middle_node());
}
