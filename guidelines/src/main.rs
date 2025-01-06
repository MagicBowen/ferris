use guidelines::ThreadPool;
fn main() {
    let pool = ThreadPool::new(4);
    for i in 0..10 {
        pool.execute(move || {
            println!("Task {}", i);
        });
    }
}
