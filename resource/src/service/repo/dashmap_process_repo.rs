use crate::domain::process::{Pid, Process};
use dashmap::DashMap;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

pub struct ProcessRepo {
    procs: DashMap<Pid, Arc<Mutex<Process>>>,
}

impl ProcessRepo {
    pub fn new() -> Self {
        ProcessRepo {
            procs: DashMap::new(),
        }
    }

    pub fn clear(&self) {
        self.procs.clear();
    }

    pub fn add_process(&self, pid: &Pid, proc: Process) -> Result<(), String> {
        if self.procs.contains_key(pid) {
            return Err(format!("Process with pid {} already exists", pid));
        }
        self.procs.insert(*pid, Arc::new(Mutex::new(proc)));
        Ok(())
    }

    pub fn get_process(&self, pid: &Pid) -> Option<Arc<Mutex<Process>>> {
        self.procs.get(pid).map(|entry| entry.value().clone())
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Pid, &Process),
    {
        self.procs.iter().for_each(|entry| {
            let pid = *entry.key();
            if let Ok(process) = entry.value().lock() {
                f(pid, &*process);
            }
        });
    }

    #[allow(dead_code)]
    pub fn for_each_mut(&self, mut f: impl FnMut(Pid, &mut Process)) {
        self.procs.iter_mut().for_each(|entry| {
            let pid = *entry.key();
            if let Ok(mut process) = entry.value().lock() {
                f(pid, &mut *process);
            }
        });
    }

    pub fn map_concurrent<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(Pid, &Process) -> R + Send + Sync,
        R: Send,
    {
        self.procs
            .par_iter()
            .filter_map(|entry| {
                let pid = *entry.key();
                match entry.value().lock() {
                    Ok(process) => Some(f(pid, &*process)),
                    Err(_) => None,
                }
            })
            .collect()
    }
}