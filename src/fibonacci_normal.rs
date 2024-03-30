use std::time::Instant;

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    let start = Instant::now(); // 実行前の時刻を取得

    for _ in 0..5 {
        let result = fib(40);
        println!("fib(40) = {}", result);
    }

    let duration = start.elapsed(); // 実行時間を計測
    println!("Total execution time: {:?}", duration);
}
