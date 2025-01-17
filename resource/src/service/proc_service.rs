use crate::domain::resource::ResourceType;
use crate::domain::allocation::AllocationFactory;
use crate::domain::process::{Pid, Process};
use super::proc_repo::ProcRepo;

pub struct ProcService {
    proc_repo: ProcRepo,
}

impl ProcService {
    pub fn new() -> Self {
        ProcService {
            proc_repo: ProcRepo::new(),
        }
    }

    pub fn add_process(&mut self, pid: Pid) -> Result<(), String> {
        self.proc_repo.add_proc(&pid, Process::new())
    }

    pub fn add_allocation(
        &mut self,
        pid: Pid,
        time: i32,
        res: ResourceType,
        capacity: i32,
    ) -> Result<(), String> {
        if let Some(proc) = self.proc_repo.get_proc_mut(&pid) {
            proc.add_allocation(AllocationFactory::create(res, capacity, time));
            return Ok(());
        }
        Err(format!("Process with pid {} not found", pid))
    }

    pub fn compute_process(&self, pid: Pid) -> Option<(i32, i32)> {
        if let Some(proc) = self.proc_repo.get_proc(&pid) {
            return Some((proc.compute_cost(), proc.compute_penalty()));
        }
        None
    }
}
