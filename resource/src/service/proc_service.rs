use crate::domain::allocation::Allocation;
use crate::domain::process::{Pid, Process};
use crate::domain::resource::{Resource, ResourceType};
use std::collections::HashMap;

pub struct ProcService {
    proc_repo: HashMap<Pid, Box<Process>>,
}

impl ProcService {
    pub fn new() -> Self {
        ProcService {
            proc_repo: HashMap::new(),
        }
    }

    pub fn add_process(&mut self, pid: Pid) -> Result<(), String> {
        if self.proc_repo.contains_key(&pid) {
            return Err(format!("Process with pid {} already exists", pid));
        }
        let proc = Box::new(Process::new());
        self.proc_repo.insert(pid, proc);
        Ok(())
    }

    pub fn add_allocation(
        &mut self,
        pid: Pid,
        time: i32,
        res: ResourceType,
        capacity: i32,
    ) -> Result<(), String> {
        if let Some(proc) = self.proc_repo.get_mut(&pid) {
            let resource = Resource::new(res, capacity);
            let allocation = Allocation::new(resource, time);
            proc.add_allocation(allocation);
            return Ok(());
        }
        Err(format!("Process with pid {} not found", pid))
    }

    pub fn compute_process(&self, pid: Pid) -> Option<(i32, i32)> {
        if let Some(proc) = self.proc_repo.get(&pid) {
            return Some((proc.compute_cost(), proc.compute_penalty()));
        }
        None
    }
}
