pub fn give_us_the_time() {
    println!("The Utc time is: {}", chrono::Utc::now());
    println!("The current date / time is: {}", chrono::Local::now());
}
