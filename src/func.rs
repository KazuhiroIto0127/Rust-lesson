fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let message: String = greet("Alice");
    println!("{}", message)
}
