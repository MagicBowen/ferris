use super::ProcessRepo;
use crate::domain::allocation::AllocationFactory;
use crate::domain::process::{Pid, Process};
use crate::domain::resource::ResourceType;

pub struct ProcService<'a> {
    proc_repo: &'a ProcessRepo,
}

impl<'a> ProcService<'a> {
    pub fn new(repo: &'a ProcessRepo) -> Self {
        ProcService { proc_repo: repo }
    }

    pub fn add_process(&self, pid: Pid) -> Result<(), String> {
        self.proc_repo.add_process(&pid, Process::new())
    }

    pub fn add_allocation(
        &self,
        pid: Pid,
        time: i32,
        res: ResourceType,
        capacity: i32,
    ) -> Result<(), String> {
        if let Some(proc) = self.proc_repo.get_process(&pid) {
            proc.lock()
                .unwrap()
                .add_allocation(AllocationFactory::create(res, capacity, time));
            return Ok(());
        }
        Err(format!("Process with pid {} not found", pid))
    }

    pub fn compute_process(&self, pid: Pid) -> Option<(i32, i32)> {
        if let Some(proc) = self.proc_repo.get_process(&pid) {
            let proc = proc.lock().unwrap();
            let cost = proc.compute_cost();
            let penalty = proc.compute_penalty();
            return Some((cost, penalty));
        }
        None
    }

    pub fn compute_all(&self) -> Vec<(Pid, i32, i32)> {
        let mut result = vec![];
        self.proc_repo.for_each(|pid, proc| {
            let cost = proc.compute_cost();
            let penalty = proc.compute_penalty();
            result.push((pid, cost, penalty));
        });
        result
    }

    pub fn compute_all_concurrent(&self) -> Vec<(Pid, i32, i32)> {
        self.proc_repo.map_concurrent(|pid, proc| {
            let cost = proc.compute_cost();
            let penalty = proc.compute_penalty();
            (pid, cost, penalty)
        })
    }
}
