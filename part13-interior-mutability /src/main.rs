use std::cell::RefCell;

struct Cat {
    times_spoken: RefCell<usize>,
}

impl Cat {
    fn report(&self) {
        println!("I spoke {} times", *self.times_spoken.borrow());
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
        *self.times_spoken.borrow_mut() += 1;
    }
}

trait Animal {
    fn speak(&self);
}

fn work_with_animal(animal: &dyn Animal) {
    animal.speak();
}

fn main() {
    let cat = Cat {
        times_spoken: RefCell::new(0),
    };
    work_with_animal(&cat);
    work_with_animal(&cat);
    work_with_animal(&cat);
    work_with_animal(&cat);
    cat.report();
}
