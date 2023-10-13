fn main() {
    let n = 42;
    println!("fib({}) = {}", n, fib(n));
}

fn fib(n: i32) -> i32 {
    if n < 2 {
        return n;
    }

    let mut x = 1;
    let mut a = 0;
    let mut b = 1;
    // while x < n {
    //     let tmp = a + b;
    //     a = b;
    //     b = tmp;
    //     x += 1;
    // }
    let answer = loop {
        let tmp = a + b;
        a = b;
        b = tmp;
        x += 1;

        if x >= n {
            break b;
        }
    };
    println!("answer is {} ", answer);
    answer
}
