
use std::sync::{Arc, Mutex};
struct Task {
    callback: Box<dyn FnOnce() + Send>,
}

impl Task {
    fn new(callback: Box<dyn FnOnce() + Send>) -> Task {
        Task { callback }
    }

    fn execute(self) {
        (self.callback)();
    }
}

struct Worker {
    id: u32,
    pub thread: std::thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<std::sync::mpsc::Receiver<Task>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let task = receiver.lock().unwrap().recv();
            match task {
                Ok(task) => task.execute(),
                Err(_) => {
                    println!("Worker {id} shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread }
    }

    fn join(self) {
        println!("worker {} join...", self.id);
        self.thread.join().unwrap();
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<std::sync::mpsc::Sender<Task>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id as u32, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender), }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: Fn() + Send + 'static,
    {
        let task = Task::new(Box::new(f));
        self.sender.as_ref().unwrap().send(task).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            worker.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_pool() {
        let pool = ThreadPool::new(4);

        for i in 0..10 {
            pool.execute(move || {
                println!("task {}", i);
            });
        }
    }
}