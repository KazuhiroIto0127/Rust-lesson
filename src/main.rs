mod func;

fn main() {
    println!("hello world");
    let message = func::greet("Alice");
    println!("{}", message);

    let sum = func::add(10, 3);
    println!("The sum is: {}", sum);

    // ============
    // if else
    // ============
    let x = 5;

    if x > 0 {
        println!("x is positive");
    } else {
        println!("x is not positive");
    }

    // ==========
    // for
    // ==========
    let numbers = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("{}", number);
    }
}
