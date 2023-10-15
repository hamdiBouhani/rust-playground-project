// fn double(n: i32) -> i32 {
//     n * 2
// }

// fn triple(n: i32) -> i32 {
//     n * 3
// }

// type MultiplierFunction = fn(i32) -> i32;

// fn make_multiplier(x: i32) -> MultiplierFunction {
//     match x {
//         2 => double,
//         3 => triple,
//         _ => unimplemented!(),
//     }
// }

// fn make_multiplier(x: i32) -> impl Fn(i32) -> i32 {
//     move |n| n * x
// }

fn make_multiplier(mut x: i32) -> impl FnMut(i32) -> i32 {
    move |n| {
        x += 1;
        n * x
    }
}

fn main() {
    let nums = [1, 2, 3];
    let nums_as_iter = nums.into_iter();

    // let multiplied = nums_as_iter.map(make_multiplier(2));
    // for n in multiplied {
    //     println!("{}", n);
    // }

    let multiplied = nums_as_iter.map(make_multiplier(2));
    for n in multiplied {
        println!("{}", n);
    }
}
