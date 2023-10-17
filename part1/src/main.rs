struct Point {
    x: i32,
    y: i32,
}

// Generics //

struct Foo<T> {
    bar: T,
}

// Methods //

impl<T> Foo<T> {
    // Methods take an explicit `self` parameter
    fn bar(&self) -> &T {
        // self is borrowed
        &self.bar
    }
    fn bar_mut(&mut self) -> &mut T {
        // self is mutably borrowed
        &mut self.bar
    }
    fn into_bar(self) -> T {
        // here self is consumed
        self.bar
    }
}

// one binary crates
fn main() {
    let origin: Point = Point { x: 0, y: 0 };

    // A struct with unnamed fields, called a ‘tuple struct’
    struct Point2(i32, i32);

    let origin2 = Point2(0, 0);

    println!("The origin is at ({}, {})", origin.x, origin.y);
    println!("The origin is at ({}, {})", origin2.0, origin2.1);

    let a_foo = Foo { bar: 1 };
    println!("{}", a_foo.bar()); // 1
}
