
use std::sync::{Arc, Mutex, mpsc};
struct Task {
    cb: Box<dyn FnOnce() + Send>,
}

impl Task {
    fn new(cb: Box<dyn FnOnce() + Send>) -> Task {
        Task { cb }
    }

    fn exec(self) {
        (self.cb)();
    }
}

struct Worker {
    id: u32,
    pub thread: std::thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Task>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let msg = receiver.lock().unwrap().recv();
            match msg {
                Ok(task) => task.exec(),
                Err(_) => {
                    println!("Worker {id} is shutting down...");
                    break;
                }
            }
        });

        Worker { id, thread }
    }

    fn join(self) {
        println!("worker {} is joining...", self.id);
        self.thread.join().unwrap();
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Task>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id as u32, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender), }
    }

    pub fn exec<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Task::new(Box::new(f));
        self.sender.as_ref().unwrap().send(task).expect("Send task failed");
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

    const THREAD_NUM: usize = 4;
    const TASK_NUM: usize = 10;

    #[test]
    fn test_thread_pool() {
        let pool = ThreadPool::new(THREAD_NUM);

        let statistics = Arc::new(Mutex::new(vec![String::new(); TASK_NUM]));

        for i in 0..TASK_NUM {
            let statistics = Arc::clone(&statistics);
            pool.exec(move || {
                let mut statistics = statistics.lock().unwrap();
                statistics[i] = format!("task {}", i);
            });
        }

        drop(pool);

        let statistics = statistics.lock().unwrap();
        assert_eq!(statistics.len(), TASK_NUM);

        for i in 0..TASK_NUM {
            assert_eq!(statistics[i], format!("task {}", i));
        }
    }
}