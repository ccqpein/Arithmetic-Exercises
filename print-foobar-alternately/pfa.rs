use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct FooBar {
    n: i64,
}

impl FooBar {
    fn foo(&self) -> impl Fn() + use<'_> {
        || {
            for _ in 0..self.n.clone() {
                print!("Foo")
            }
        }
    }

    fn bar(&self) -> impl Fn() + use<'_> {
        || {
            for _ in 0..self.n.clone() {
                print!("Bar\n")
            }
        }
    }
}

// fn way1() {
//     let fb = Arc::new(Mutex::new(FooBar { n: 5 }));

//     let handle1 = thread::spawn(|| {
//         fb.foo();
//     });

//     // Spawn the second thread
//     let handle2 = thread::spawn(|| {
//         fb.bar();
//     });

//     handle1.join().unwrap();
//     handle2.join().unwrap();
// }

// this way isn't right
fn way2() {
    let fb = FooBar { n: 5 };
    thread::scope(|s| {
        s.spawn(|| {
            fb.foo()();
        });
        s.spawn(|| {
            let a = fb.bar();
            a()
        });
    });
}

fn main() {
    //way2();
}
