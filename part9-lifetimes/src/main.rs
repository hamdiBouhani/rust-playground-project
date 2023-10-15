struct OurIterator<'a> {
    nums: &'a [i32],
    i: usize,
    x: i32,
}

impl<'a> OurIterator<'a> {
    fn new(nums: &'a [i32], x: i32) -> Self {
        //[i32] it's like view inside of Vec<i32> to data
        Self { nums, i: 0, x }
    }
}

impl Iterator for OurIterator<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.nums.len() {
            let val = self.nums[self.i];
            self.i += 1;
            Some(self.x * val)
        } else {
            None
        }
    }
}

impl Drop for OurIterator<'_> {
    fn drop(&mut self) {
        println!("Dropping iterator");
    }
}

#[allow(clippy::while_let_on_iterator)]
fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];

    let mut iter1 = OurIterator::new(&nums, 2);
    let mut iter2 = OurIterator::new(&nums, 3);

    while let Some(num) = iter1.next() {
        println!("{}", num);
    }
    drop(iter1);
    while let Some(num) = iter2.next() {
        println!("{}", num);
    }
    drop(iter2);
    
    nums.push(6);
}
