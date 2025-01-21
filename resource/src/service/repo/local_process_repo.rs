use crate::domain::process::{Pid, Process};
use std::collections::HashMap;
use std::cell::RefCell;
use rayon::prelude::*;

pub struct LocalMutex<T>(RefCell<T>);

impl<T> LocalMutex<T> {
    pub fn new(value: T) -> Self {
        Self(RefCell::new(value))
    }

    pub fn lock(&mut self) -> Result<&mut T, ()> {
        Ok(self.0.get_mut())
    }
}

pub struct ProcessRepo {
    procs: RefCell<HashMap<Pid, LocalMutex<Process>>>,
}

impl ProcessRepo {
    pub fn new() -> Self {
        ProcessRepo {
            procs: RefCell::new(HashMap::new()),
        }
    }

    pub fn clear(&self) {
        self.procs.borrow_mut().clear();
    }

    pub fn add_process(&self, pid: &Pid, proc: Process) -> Result<(), String> {
        if self.procs.borrow().contains_key(pid) {
            return Err(format!("Process with pid {} already exists", pid));
        }
        self.procs.borrow_mut().insert(*pid, LocalMutex::new(proc));
        Ok(())
    }

    pub fn get_process(&self, pid: &Pid) -> Option<&LocalMutex<Process>> {
        self.procs.borrow().get(pid)
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Pid, &Process),
    {
        let procs = self.procs.borrow();

        procs.iter().for_each(|(&pid, process)| {
            f(pid, process.lock().unwrap());
        });
    }

    #[allow(dead_code)]
    pub fn for_each_mut(&self, mut f: impl FnMut(Pid, &mut Process)) {
        let mut procs = self.procs.borrow_mut();

        procs.iter_mut().for_each(|(pid, process)| {
            f(*pid, &mut process.lock().unwrap());
        });
    }

    pub fn map_concurrent<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(Pid, &Process) -> R + Send + Sync,
        R: Send,
    {
        let procs = self.procs.borrow();

        procs.par_iter().map(|(&pid, process)| f(pid, process.lock().unwrap())).collect()
    }
}