use crate::ResourceType;
use crate::service::ProcService;
use crate::service::ProcessRepo;

use std::cell::RefCell;

thread_local! {
    static PROCESS_REPO: RefCell<ProcessRepo> = RefCell::new(ProcessRepo::new());
    static PROC_SERVICE: RefCell<ProcService<'static>> = RefCell::new(ProcService::new(PROCESS_REPO.with_borrow(|r| r)));
}

pub fn config_process(pid: u32) -> Result<(), String> {
    PROC_SERVICE.with_borrow(|s| s.add_process(pid))
}

pub fn config_allocation(
    pid: u32,
    time: i32,
    res: ResourceType,
    capacity: i32,
) -> Result<(), String> {
    PROC_SERVICE.with_borrow(|s| s.add_allocation(pid, time, res, capacity))
}

pub fn compute_process(pid: u32) -> Option<(i32, i32)> {
    PROC_SERVICE.with_borrow(|s| s.compute_process(pid))
}

pub fn compute_all() -> Vec<(u32, i32, i32)> {
    PROC_SERVICE.with_borrow(|s| s.compute_all())
}

pub fn compute_all_concurrent() -> Vec<(u32, i32, i32)> {
    PROC_SERVICE.with_borrow(|s| s.compute_all_concurrent())
}