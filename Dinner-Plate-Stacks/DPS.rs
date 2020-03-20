struct DinnerPlates {
    cap: i32,
    stacks: Vec<Vec<i32>>,

    //
    stacks_num: usize,

    //
    push_point: usize,
    pop_point: usize,
}

impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        DinnerPlates {
            cap: capacity,
            stacks: vec![vec![]],
            stacks_num: 1,
            push_point: 0,
            pop_point: 0,
        }
    }

    fn push(&mut self, val: i32) {
        if self.stacks[self.push_point].len() < self.cap as usize {
            self.stacks[self.push_point].push(val);
            return;
        }

        loop {
            if self.push_point == self.stacks_num - 1 {
                self.stacks.push(vec![]);
                self.stacks_num += 1;

                // only here
                self.pop_point += 1;
            }
            self.push_point += 1;
            if self.push_point > self.pop_point {
                self.pop_point = self.push_point;
            }
            if self.stacks[self.push_point].len() < self.cap as usize {
                self.stacks[self.push_point].push(val);
                return;
            }
        }
    }

    fn pop(&mut self) -> i32 {
        for i in (0..=self.pop_point).rev() {
            if self.stacks[i].len() != 0 {
                self.pop_point = i;
                return self.pop_at_stack(i as i32);
            }
        }
        return -1;
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        if index as usize >= self.stacks_num {
            return -1;
        }
        match self.stacks[index as usize].pop() {
            Some(r) => {
                if (index as usize) < self.push_point {
                    self.push_point = index as usize;
                };
                return r;
            }
            None => return -1,
        };
    }
}

fn test(op: Vec<&str>, nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut test_instance = DinnerPlates::new(0);
    let mut result = vec![];
    for i in 0..op.len() {
        match op[i] {
            "DinnerPlates" => {
                test_instance = DinnerPlates::new(nums[i][0]);
            }
            "push" => {
                test_instance.push(nums[i][0]);
                println!("{:?}", test_instance.stacks);
            }
            "popAtStack" => {
                result.push(test_instance.pop_at_stack(nums[i][0]));
                println!("{:?}", test_instance.stacks);
            }
            "pop" => {
                result.push(test_instance.pop());
                println!("{:?}", test_instance.stacks);
            }
            _ => (),
        }
    }
    result
}

fn main() {
    dbg!(test(
        vec![
            "DinnerPlates",
            "push",
            "push",
            "push",
            "push",
            "push",
            "push",
            "push",
            "popAtStack",
            "popAtStack",
            "popAtStack",
            "popAtStack",
            "popAtStack",
            "push",
            "push",
            "pop",
            "pop",
            "pop",
            "pop",
            "pop",
        ],
        vec![
            vec![2],
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![5],
            vec![6],
            vec![7],
            vec![2],
            vec![2],
            vec![1],
            vec![1],
            vec![0],
            vec![8],
            vec![9],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ],
    ));

    dbg!(test(
        vec![
            "DinnerPlates",
            "push",
            "push",
            "popAtStack",
            "pop",
            "push",
            "push",
            "pop",
            "pop"
        ],
        vec![
            vec![1],
            vec![1],
            vec![2],
            vec![1],
            vec![],
            vec![1],
            vec![2],
            vec![],
            vec![]
        ]
    ));
}
