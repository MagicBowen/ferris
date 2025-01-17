use std::collections::HashMap;
use crate::domain::process::{Pid, Process};

pub struct ProcessRepo {
    procs: HashMap<Pid, Process>,
}

impl ProcessRepo {
    pub fn new() -> Self {
        ProcessRepo {
            procs: HashMap::new(),
        }
    }

    pub fn add_process(&mut self, pid: &Pid, proc: Process) -> Result<(), String> {
        if self.procs.contains_key(&pid) {
            return Err(format!("Process with pid {} already exists", pid));
        }
        self.procs.insert(*pid, proc);
        Ok(())
    }

    pub fn get_process(&self, pid: &Pid) -> Option<&Process> {
        self.procs.get(&pid)
    }

    pub fn get_process_mut(&mut self, pid: &Pid) -> Option<&mut Process> {
        self.procs.get_mut(&pid)
    }
}