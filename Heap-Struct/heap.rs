use std::boxed::Box;
use std::fmt;
use std::fmt::Debug;

struct Heap<T> {
    inner_vec: Vec<T>,
    cap: usize,
    insert_p: usize,

    /// cmp func used in shift_up of insert
    /// and shift_down of delete
    cmp_func: Box<dyn Fn(&T, &T) -> bool>,
}

impl<T> Debug for Heap<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Heap")
            .field("inner_vec", &self.inner_vec)
            .field("cap", &self.cap)
            .field("insert_p", &self.insert_p)
            .finish()
    }
}

impl<T> Heap<T>
where
    T: Clone + Default,
{
    fn new(f: Box<dyn Fn(&T, &T) -> bool>) -> Self {
        Self {
            inner_vec: vec![Default::default(); 2], // first one take index 0
            cap: 2,
            insert_p: 1, // next insert index
            cmp_func: f,
        }
    }

    fn insert(&mut self, ele: &T) {
        if self.insert_p == self.cap {
            self.double_cap();
        }
        self.inner_vec[self.insert_p] = ele.clone();

        self.shift_up(self.insert_p);

        self.insert_p += 1;
    }

    fn delete(&mut self) -> Option<T> {
        if self.insert_p == 1 {
            return None;
        }

        let result = Some(self.inner_vec[1].clone());
        self.insert_p -= 1;
        self.inner_vec[1] = self.inner_vec[self.insert_p].clone();
        self.inner_vec[self.insert_p] = Default::default();

        self.shift_down(1);

        result
    }

    /*
    util functions below
     */

    fn double_cap(&mut self) {
        self.inner_vec
            .append(&mut vec![Default::default(); self.cap - 1]);
        self.cap *= 2;
        self.cap -= 1;
    }

    fn shift_up(&mut self, ind: usize) {
        let root = (ind as i32 / 2) as usize;
        if root != 0 && !self.cmp_root_child(root, ind) {
            self.swop(root, ind);
            self.shift_up(root)
        }
    }

    fn shift_down(&mut self, ind: usize) {
        let (left, right) = (ind * 2, ind * 2 + 1);

        // last branch
        if right >= self.insert_p {
            self.swop(ind, left);
            return;
        }

        let next_ind = if self.cmp_root_child(left, right) {
            self.swop(ind, left);
            left
        } else {
            self.swop(ind, right);
            right
        };

        self.shift_down(next_ind);
    }

    fn swop(&mut self, a: usize, b: usize) {
        let aa = self.inner_vec[a].clone();
        self.inner_vec[a] = self.inner_vec[b].clone();
        self.inner_vec[b] = aa
    }

    /// id1 and id2 have to exsit
    fn cmp_root_child(&self, id1: usize, id2: usize) -> bool {
        self.cmp_func.as_ref()(&self.inner_vec[id1], &self.inner_vec[id2])
    }
}

fn main() {
    let mut heap = Heap::new(Box::new(|a: &i32, b: &i32| a <= b));
    for i in vec![10, 99, 14, 25, 33, 81, 82] {
        heap.insert(&i);
    }

    dbg!(&heap);
    dbg!(heap.delete());
    dbg!(&heap);

    heap.insert(&79);
    dbg!(&heap);
}
