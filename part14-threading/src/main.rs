fn main() {
    println!("Hello, world!");
    let steps = 5;
    let thread = std::thread::spawn(move || {
        for i in 1..steps {
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("Thread steps {}", i);
        }
        "Goodbye!"
    });
    println!("Spawned a thread");
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("Slept for 2 seconds. Now Joining thread...");
    let result = thread.join().unwrap(); // wait for thread to finish
    println!("Thread returned with {:?}", result);
}
