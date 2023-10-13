fn main() {
    printing::say_hello();
    printing::say_goodbye();

}


mod printing {
    pub fn say_hello() {
        println!("Hello, world!");
    }

    pub fn say_goodbye() {
        println!("Goodbye, world!");
    }
}

