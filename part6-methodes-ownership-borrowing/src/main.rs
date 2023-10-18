use model::Hands;

mod model {
    pub struct Item {
        what: String,
        present: bool,
    }

    pub struct Hands {
        left: Item,
        right: Item,
    }

    impl Hands {
        pub fn new() -> Self {
            let  hands = Hands {
                left: Item {
                    what: "an apple".to_owned(),
                    present: true,
                },
                right: Item {
                    what: "an banana".to_owned(),
                    present: true,
                },
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
            Item::report_item(&self.left, "Left");
            Item::report_item(&self.right, "Right");
        }
    }

    impl Item {
        pub fn report_item(&self, which: &str) {
            if self.present {
                println!("{} hand is holding {}", which, self.what);
            } else {
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
