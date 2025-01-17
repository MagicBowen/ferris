use std::collections::HashMap;
use crate::domain::process::{Pid, Process};

pub struct ProcRepo {
    procs: HashMap<Pid, Process>,
}

impl ProcRepo {
    pub fn new() -> Self {
        ProcRepo {
            procs: HashMap::new(),
        }
    }

    pub fn add_proc(&mut self, pid: &Pid, proc: Process) -> Result<(), String> {
        if self.procs.contains_key(&pid) {
            return Err(format!("Process with pid {} already exists", pid));
        }
        self.procs.insert(*pid, proc);
        Ok(())
    }

    pub fn get_proc(&self, pid: &Pid) -> Option<&Process> {
        self.procs.get(&pid)
    }

    pub fn get_proc_mut(&mut self, pid: &Pid) -> Option<&mut Process> {
        self.procs.get_mut(&pid)
    }
}