use std::{
    io::{self, Write},
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

// cannot happen
// fn way1() {
//     let fb = Arc::new(Mutex::new(FooBar { n: 5 }));

//     let fb0 = Arc::clone(&fb);
//     let handle1 = thread::spawn(move || {
//         match fb0.lock() {
//             Ok(ff) => ff.foo()(),
//             Err(_) => todo!(),
//         };
//     });

//     let fb1 = Arc::clone(&fb);
//     let handle2 = thread::spawn(move || {
//         match fb1.lock() {
//             Ok(bb) => bb.bar()(),
//             Err(_) => todo!(),
//         };
//     });

//     handle1.join().unwrap();
//     handle2.join().unwrap();
// }

// this way isn't right
// fn way2() {
//     let fb = FooBar { n: 5 };
//     thread::scope(|s| {
//         s.spawn(|| {
//             fb.foo()();
//         });
//         s.spawn(|| {
//             let a = fb.bar();
//             a()
//         });
//     });
// }

// way 3 kind of do the thing
// lets do with this template
fn way3() {
    let handle1 = thread::spawn(move || {
        for _ in 0..5 {
            print!("Foo");
            io::stdout().flush().unwrap(); // Explicitly flush stdout
            thread::sleep(Duration::from_millis(10)); // Yield CPU
        }
    });

    let handle2 = thread::spawn(move || {
        for _ in 0..5 {
            print!("Bar\n");
            io::stdout().flush().unwrap(); // Explicitly flush stdout
            thread::sleep(Duration::from_millis(10)); // Yield CPU
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn way4() {
    let (fsender, freceiver) = std::sync::mpsc::channel::<()>();
    let (bsender, breceiver) = std::sync::mpsc::channel::<()>();

    fsender.send(());
    let handle1 = thread::spawn(move || {
        for _ in 0..5 {
            freceiver.recv();
            print!("Foo");
            io::stdout().flush().unwrap(); // Explicitly flush stdout
            thread::sleep(Duration::from_millis(10)); // Yield CPU

            bsender.send(());
        }
    });

    let handle2 = thread::spawn(move || {
        for _ in 0..5 {
            breceiver.recv();
            print!("Bar\n");
            io::stdout().flush().unwrap(); // Explicitly flush stdout
            thread::sleep(Duration::from_millis(10)); // Yield CPU
            fsender.send(());
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    //way2();
    //way1()
    way3();
    println!("way3 done");
    way4();
}
