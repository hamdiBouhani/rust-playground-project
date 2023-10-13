fn main() {
    printing::say_hello();
    printing::announce_time();
    printing::say_goodbye();
}

mod printing {
    pub mod time_stuff;
    pub use time_stuff::give_us_the_time as announce_time;

    pub fn say_hello() {
        println!("Hello, world!");
    }

    pub fn say_goodbye() {
        println!("Goodbye, world!");
    }
}
