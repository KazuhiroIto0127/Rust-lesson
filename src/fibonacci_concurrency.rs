use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    let start = Instant::now(); // 実行前の時刻を取得
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx)); // チャネルのレシーバを複数のスレッドで共有する

    for _ in 0..5 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(fib(40)).unwrap();
            drop(tx); // このスレッドの送信者をドロップ
        });
    }

    let handler = thread::spawn(move || {
        let rx = rx.lock().unwrap();
        for result in rx.iter() {
            println!("{}", result);
        }
    });

    drop(tx); // 全ての送信者をドロップして、レシーバのイテレーションを終了させる

    handler.join().unwrap(); // 結果を出力するスレッドの終了を待つ

    let duration = start.elapsed(); // 実行時間を計測
    println!("Execution time: {:?}", duration);
}
