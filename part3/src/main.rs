fn main() {
    let x: u64 = 5;
    let y = -6;
    let xy = x as i64 * y;

    println!("{} * {} = {}", x, y, xy);

    let x1: f32 = 5.1;
    let y1 = -6;
    let xy1 = x1 * y1 as f32;

    println!("{} * {} = {}", x1, y1, xy1);

    let xy_is_positive = xy > 0;
    println!("Is {} positive? {}", xy, xy_is_positive);

    let favorite = '🐶';
    println!("My favorite character is {}.", favorite);

    let favorite_bits = favorite as u32;
    println!("My favorite character in bits U+{:x}.", favorite_bits);

    let x_and_y = (5.1, -6); // x_and_y: (f32, i32)
    println!("The tuple is x_and_y = {:?}", x_and_y);

    let x = x_and_y.0;
    let y = x_and_y.1;
    println!("x = {}, y = {}", x, y);

    let x_and_y1 = [5.1, -6.0]; // should be in the same type
    println!("The tuple is x_and_y = {:?}", x_and_y1);

    let x1 = x_and_y1[0];
    let y1 = x_and_y1[1];
    println!("x = {}, y = {}", x1, y1);

    #[derive(Debug)]
    struct Secrets {
        x: f64,
        y: i32,
    }

    let x_and_y2 = Secrets { x: 5.1, y: -6 };
    println!("The tuple is x_and_y = {:?}", x_and_y2);

    let x2 = x_and_y2.x;
    let y2 = x_and_y2.y;
    println!("x = {}, y = {}", x2, y2);

    #[allow(dead_code)]
    enum Fruit {
        Apple,
        Banana,
        Orange,
    }

    #[allow(unused_variables)]
    let fruit = Fruit::Apple;

    let func = say_hello;
    func();

    let nothing = ();
    type Food = Fruit;
    let some_food: Food = Fruit::Apple;
}

fn say_hello() {
    println!("Hello, world!");
}
