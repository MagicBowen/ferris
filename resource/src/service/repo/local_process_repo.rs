use crate::domain::process::{Pid, Process};
use rayon::prelude::*;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct ProcessHandle(Rc<RefCell<Process>>);

impl ProcessHandle {
    pub fn lock(&self) -> Option<RefMut<Process>> {
        Some(self.0.borrow_mut())
    }
}

unsafe impl Send for ProcessHandle {}
unsafe impl Sync for ProcessHandle {}

pub struct ProcessRepo {
    procs: RefCell<HashMap<Pid, ProcessHandle>>,
}

impl ProcessRepo {
    pub fn new() -> Self {
        ProcessRepo {
            procs: RefCell::new(HashMap::new()),
        }
    }

    pub fn add_process(&self, pid: &Pid, proc: Process) -> Result<(), String> {
        if self.procs.borrow().contains_key(pid) {
            return Err(format!("Process with pid {} already exists", pid));
        }
        self.procs
            .borrow_mut()
            .insert(*pid, ProcessHandle(Rc::new(RefCell::new(proc))));
        Ok(())
    }

    pub fn get_process(&self, pid: &Pid) -> Option<ProcessHandle> {
        let procs = self.procs.borrow();
        procs.get(pid).cloned()
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Pid, &Process),
    {
        let procs = self.procs.borrow();

        procs.iter().for_each(|(pid, process)| {
            f(*pid, &process.0.borrow());
        });
    }

    #[allow(dead_code)]
    pub fn for_each_mut(&self, mut f: impl FnMut(Pid, &mut Process)) {
        let mut procs = self.procs.borrow_mut();

        procs.iter_mut().for_each(|(pid, process)| {
            f(*pid, &mut process.0.borrow_mut());
        });
    }

    pub fn map_concurrent<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(Pid, &Process) -> R + Send + Sync,
        R: Send,
    {
        let procs = self.procs.borrow();

        procs
            .par_iter()
            .map(|(&pid, process)| f(pid, &process.0.borrow()))
            .collect()
    }
}

#[cfg(test)]
use super::*;
use crate::domain::allocation::AllocationFactory;
use crate::ResourceType;

#[test]
fn could_exec_CRUD_from_repo() {
    let repo = ProcessRepo::new();
    let pid = 0;

    let mut proc = Process::new(pid);
    proc.add_allocation(AllocationFactory::create(ResourceType::CPU, 2, 3));
    repo.add_process(&pid, proc).expect("Could add process");

    let pid = 1;
    let mut proc = Process::new(pid);
    proc.add_allocation(AllocationFactory::create(ResourceType::CPU, 4, 2));
    repo.add_process(&pid, proc).expect("Could add process");

    let result =
        repo.map_concurrent(|pid, proc| (pid, proc.compute_cost(), proc.compute_penalty()));

    assert_eq!(result.len(), 2);
}
