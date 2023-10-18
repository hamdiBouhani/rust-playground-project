use model::Hands;

mod model {

    use std::fmt::Display;
    // pub trait Displayable {
    //     fn display(&self) -> String;
    // }

    enum Fruit {
        Apple,
        Banana,
        Kiwi,
    }

    // impl Displayable for Fruit {
    //     fn display(&self) -> String {
    //         match self {
    //             Fruit::Apple => "Apple".to_string(),
    //             Fruit::Banana => "Banana".to_string(),
    //             Fruit::Kiwi => "Kiwi".to_string(),
    //             // _ => "Unknown".to_string(),
    //         }
    //     }
    // }

    impl Display for Fruit {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Fruit::Apple => write!(f, "Apple"),
                Fruit::Banana => write!(f, "Banana"),
                Fruit::Kiwi => write!(f, "Kiwi"),
                // _ => write!(f, "Unknown"),
            }
        }
    }

    pub enum Item<T> {
        Something(T),
        Nothing,
    }

    pub struct Hands {
        left: Option<Fruit>,
        right: Option<Fruit>,
    }

    impl Hands {
        pub fn new() -> Self {
            let hands = Hands {
                left: Option::Some(Fruit::Apple),
                right: Option::Some(Fruit::Banana),
            };
            hands
        }

        #[allow(clippy::manual_swap)]
        pub fn swap_hands(mut self) -> Self {
            println!("Swapping hands");
            let temp = self.left;
            self.left = self.right;
            self.right = temp;
            self
        }

        pub fn report(&self) {
            report_item(&self.left, "Left");
            report_item(&self.right, "Right");
        }
    }

    pub fn report_item<T: Display>(item:&Option<T>, which: &str) {
        match item {
            Option::Some(what) => {
                println!("{} hand is holding {}", which, what);
            }
            _ => {
                println!("{} hand is empty", which);
            }
        }
    }
}

fn main() {
    let mut hands = Hands::new();

    hands.report();

    hands = hands.swap_hands();

    hands.report();
}
