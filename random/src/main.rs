use std::ops::Add;
use std::time::Duration;

fn main() {
    let a: i32 = 10;
    let b: i32 = 20;
    let res: i32 = add_with_lifetime(&a, &b);
    println!("{}", res);

    let floats= add(1.2, 2.4);
    let ints = add(1, 2);
    let durations = add(
        Duration::new(5, 0),
        Duration::new(10, 0)
    );

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);
}

fn add_with_lifetime<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}