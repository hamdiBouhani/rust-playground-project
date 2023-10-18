fn main() {
    let mut message = String::from("Hello, world!");

    // Concatenate another string
    message.push_str(" Welcome to Rust!");
    println!("{}", message);

    // Convert to uppercase
    let uppercase = message.to_uppercase();
    println!("{}", uppercase);

    // Split into words
    let words: Vec<&str> = message.split(' ').collect();
    for word in words {
        println!("{}", word);
    }

    // Replace a substring
    let replaced = message.replace("world", "Rust");
    println!("{}", replaced);

    // Check for a substring
    let contains_rust = message.contains("Rust");
    println!("Contains 'Rust': {}", contains_rust);

    let mut text = String::from("   Hello, Rust!   ");
    text = text.trim().to_owned(); // Removes leading and trailing whitespace
    println!("Trimmed: {}", text);
}
