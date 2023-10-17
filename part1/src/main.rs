struct Point {
    x: i32,
    y: i32,
}

// one binary crates
fn main() {
    let origin: Point = Point { x: 0, y: 0 };

    // A struct with unnamed fields, called a ‘tuple struct’
    struct Point2(i32, i32);

    let origin2 = Point2(0, 0);

    println!("The origin is at ({}, {})", origin.x, origin.y);
    println!("The origin is at ({}, {})", origin2.0, origin2.1);
}
