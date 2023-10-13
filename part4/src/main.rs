fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = 10;
    let answer = find_answer(x, y);
    println!("The answer is {}", answer);
}

fn find_answer(x: i32, y: i32) -> i32 {
    x + y * 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_answer() {
        assert_eq!(find_answer(2, 3), 17);
    }
}
