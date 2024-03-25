mod func;

fn main() {
    println!("hello world");
    let message = func::greet("Alice");
    println!("{}", message);

    let sum = func::add(10, 3);
    println!("The sum is: {}", sum);
}
