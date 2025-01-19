use crate::domain::process::{Pid, Process};
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

pub struct ProcessRepo {
    procs: RwLock<HashMap<Pid, Arc<Mutex<Process>>>>,
}

impl ProcessRepo {
    pub fn new() -> Self {
        ProcessRepo {
            procs: RwLock::new(HashMap::new()),
        }
    }

    pub fn clear(&self) {
        let mut procs = self.procs.write().unwrap();
        procs.clear();
    }

    pub fn add_process(&self, pid: &Pid, proc: Process) -> Result<(), String> {
        let mut procs = self.procs.write().unwrap();

        if procs.contains_key(&pid) {
            return Err(format!("Process with pid {} already exists", pid));
        }
        procs.insert(*pid, Arc::new(Mutex::new(proc)));
        Ok(())
    }

    pub fn get_process(&self, pid: &Pid) -> Option<Arc<Mutex<Process>>> {
        let procs = self.procs.read().unwrap();
        procs.get(pid).map(|p| p.clone())
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Pid, &Process),
    {
        let procs = self.procs.read().unwrap();

        procs.iter().for_each(|(&pid, proc_mutex)| {
            if let Ok(process) = proc_mutex.lock() {
            f(pid, &*process);
            }
        });
    }

    #[allow(dead_code)]
    pub fn for_each_mut(&self, mut f: impl FnMut(Pid, &mut Process)) {
        let procs = self.procs.read().unwrap();

        procs.iter().for_each(|(&pid, proc_mutex)| {
            if let Ok(mut process) = proc_mutex.lock() {
            f(pid, &mut *process);
            }
        });
    }

    pub fn map_concurrent<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(Pid, &Process) -> R + Send + Sync,
        R: Send,
    {
        let procs_snapshot: Vec<(Pid, Arc<Mutex<Process>>)> = {
            let procs = self.procs.read().unwrap();
            procs
                .iter()
                .map(|(&pid, proc_mutex)| (pid, Arc::clone(proc_mutex)))
                .collect()
        };

        procs_snapshot
            .par_iter()
            .filter_map(|(pid, proc_mutex)| match proc_mutex.lock() {
                Ok(process) => Some(f(*pid, &*process)),
                Err(_) => None,
            })
            .collect()
    }
}
