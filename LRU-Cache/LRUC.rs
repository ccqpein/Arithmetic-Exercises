use std::collections::HashMap;

#[derive(Debug)]
struct LRUCache {
    cap: i32,

    record: Vec<i32>,
    backet: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            cap: capacity,
            record: vec![],
            backet: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(v) = self.backet.get(&key) {
            let mut head = vec![key];
            let mut tail = vec![];
            //let mut find = false;
            for vv in self.record.clone() {
                if vv != key {
                    tail.push(vv);
                }
            }

            head.append(&mut tail);
            self.record = head;
            return v.clone();
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        //let v = self.get(key);
        if self.get(key) != -1 {
            self.backet.insert(key, value);
            return;
        }

        let mut new = vec![key];

        if self.record.len() == self.cap as usize {
            self.backet.remove(self.record.last().unwrap());
            for i in self.record.drain(..self.record.len() - 1) {
                new.push(i);
            }
            self.record = new;
        } else {
            for i in self.record.clone() {
                new.push(i);
            }
            self.record = new;
        }
        self.backet.insert(key, value);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

fn main() {
    let mut obg = LRUCache::new(2);
    println!("{}", obg.get(2));
    obg.put(1, 1);
    obg.put(2, 2);
    println!("{}", obg.get(1));
    obg.put(3, 3);
    //println!("{:#?}", obg);
    println!("{}", obg.get(2));
    obg.put(4, 4);
    println!("{}", obg.get(1));
    println!("{}", obg.get(3));
    println!("{}", obg.get(4));

    let mut obg1 = LRUCache::new(1);
    obg1.put(2, 1);
    println!("{}", obg1.get(2));
    println!("{:#?}", obg1);
    obg1.put(3, 2);
    println!("{:#?}", obg1);
    println!("{}", obg1.get(2));
    println!("{}", obg1.get(3));
}
