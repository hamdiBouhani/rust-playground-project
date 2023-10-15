// use std::alloc::(alloc, Layout);

// fn allocate_numbers() -> Box<SomeHugeType> {
//     unsafe {
//         let p = alloc(Layout::new::<SomeHugeType>()) as *mut SomeHugeType;
//         Box::from_raw(p)
//     }
// }

// fn generate_number_on_heap() -> Box<SomeHugeType> {
//     let mut numbers = allocate_numbers();
//     for (i, item) in numbers.iter_mut().enumerate() {
//         *item = (i * 2) as i32;
//     }
//     numbers
// }

// const ANSWER: u32 = 42;
// static mut QUESTION: String = String::new();

// type SomeHugeType = [i32; 10_000_000];

// fn generate_number() -> SomeHugeType {
//     let mut numbers = [0; 10_000_000];
//     for (i, item) in numbers.iter_mut().enumerate() {
//         *item = (i * 2) as i32;
//     }
//     numbers
// }

// fn main() {
//     unsafe {
//         QUESTION = "What is the answer?".to_string();
//         println!("QUESTION: {:?}", QUESTION);
//     }
//     println!("ANSWER: {:?}", ANSWER);

//     let numbers = generate_number_on_heap();
//     for i in 0..5 {
//         println!("numbers[{}] = {}", i, numbers[i]);
//     }
// }

// -------------------------------------------------------------------

// struct Node {
//     value: i32,
//     left: Option<Box<Node>>,
//     right: Option<Box<Node>>,
// }

// impl Node {
//     fn display(&self) {
//         println!("value: {}", self.value);
//         if let Some(left) = &self.left {
//             println!("left");
//             left.display();
//         }
//         if let Some(right) = &self.right {
//             println!("right");
//             right.display();
//         }
//     }
// }

// fn main() {
//     let root = Node {
//         value: 0,
//         left: Some(Box::new(Node {
//             value: 1,
//             left: None,
//             right: None,
//         })),
//         right: Some(Box::new(Node {
//             value: 2,
//             left: Some(Box::new(Node {
//                 value: 1,
//                 left: None,
//                 right: None,
//             })),
//             right: None,
//         })),
//     };
//     root.display();
// }

// -------------------------------------------------------------------

use std::error::Error;

struct Cat {}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

struct Dog {}

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

trait Animal {
    fn speak(&self);
}

fn work_with_animal(animals: &[Box<dyn Animal>]) -> Result<(), Box<dyn Error>> {
    for animal in animals {
        animal.speak();
    }
    Ok(())
}

fn main() {
    let cat = Cat {};
    let dog = Dog {};
    cat.speak();
    dog.speak();

    //let animals: Vec<Box<dyn Animal>> = vec![Box::new(cat), Box::new(dog)];
    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(cat));
    animals.push(Box::new(dog));
    work_with_animal(&animals).unwrap()
}
