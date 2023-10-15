// use std::sync::{mpsc::channel, Arc};

// fn main() {
//     println!("Hello, world!");
//     // let steps = Arc::new(5); //Atomic Reference Counted
//     // let thread = {
//     //     let steps = steps.clone();
//     //     std::thread::spawn(move || {
//     //         for i in 1..*steps {
//     //             std::thread::sleep(std::time::Duration::from_secs(1));
//     //             println!("Thread steps {}", i);
//     //         }
//     //         "Goodbye!"
//     //     })
//     // };
//     let (sender, receiver) = channel();
//     let thread = std::thread::spawn(move || {
//         let steps = receiver.recv().unwrap();
//         for i in 1..=steps {
//             std::thread::sleep(std::time::Duration::from_secs(1));
//             println!("Thread steps {}", i);
//         }
//         "Goodbye!"
//     });
//     println!("Spawned a thread !");
//     let _ = sender.send(5);
//     std::thread::sleep(std::time::Duration::from_secs(2));
//     println!("Slept for 2 seconds. Now Joining thread...");
//     let result = thread.join().unwrap(); // wait for thread to finish
//     println!("Thread returned with {:?}", result);
// }

use std::{
    cell::RefCell,
    sync::{mpsc::channel, Arc, Mutex},
};

fn main() {
    println!("Hello, world!");
    let steps = Arc::new(Mutex::new(5)); //Atomic Reference Counted
    let thread = {
        let steps = steps.clone();
        std::thread::spawn(move || {
            while *steps.lock().unwrap() > 0 {
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Thread steps {}", steps.lock().unwrap());
                *steps.lock().unwrap() -= 1;
            }
            "Goodbye!"
        })
    };

    println!("Spawned a thread !");
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("Slept for 2 seconds. Now Joining thread...");
    let result = thread.join().unwrap(); // wait for thread to finish
    println!("Thread returned with {:?}", result);
}
