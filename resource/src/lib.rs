mod domain;
mod service;

pub use domain::resource::ResourceType;
use service::ProcService;
use service::ProcessRepo;

use lazy_static::lazy_static;

lazy_static! {
    static ref PROCESS_REPO: ProcessRepo = ProcessRepo::new();
    static ref PROC_SERVICE: ProcService<'static> = ProcService::new(&PROCESS_REPO);
}

pub fn config_process(pid: u32) -> Result<(), String> {
    PROC_SERVICE.add_process(pid)
}

pub fn config_allocation(
    pid: u32,
    time: i32,
    res: ResourceType,
    capacity: i32,
) -> Result<(), String> {
    PROC_SERVICE.add_allocation(pid, time, res, capacity)
}

pub fn compute_process(pid: u32) -> Option<(i32, i32)> {
    PROC_SERVICE.compute_process(pid)
}

pub fn compute_all() -> Vec<(u32, i32, i32)> {
    PROC_SERVICE.compute_all()
}

pub fn compute_all_concurrent() -> Vec<(u32, i32, i32)> {
    PROC_SERVICE.compute_all_concurrent()
}

pub fn clear_all() {
    PROCESS_REPO.clear();
}