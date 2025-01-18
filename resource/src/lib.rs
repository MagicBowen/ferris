mod domain;
mod service;

pub use domain::resource::ResourceType;
use service::proc_service::ProcService;
use std::cell::RefCell;

thread_local! {
    static PROC_SERVICE: RefCell<ProcService> = RefCell::new(ProcService::new());
}

pub fn config_process(pid: u32) -> Result<(), String> {
    PROC_SERVICE.with(|proc_service| proc_service.borrow_mut().add_process(pid))
}

pub fn config_allocation(
    pid: u32,
    time: i32,
    res: ResourceType,
    capacity: i32,
) -> Result<(), String> {
    PROC_SERVICE.with(|proc_service| {
        proc_service
            .borrow()
            .add_allocation(pid, time, res, capacity)
    })
}

pub fn compute_process(pid: u32) -> Option<(i32, i32)> {
    PROC_SERVICE.with(|proc_service| {
        proc_service.borrow().compute_process(pid)
    })
}

pub fn compute_all() -> Vec<(u32, i32, i32)> {
    PROC_SERVICE.with(|proc_service| {
        proc_service.borrow().compute_all()
    })
}

pub fn compute_all_concurrent() -> Vec<(u32, i32, i32)> {
    PROC_SERVICE.with(|proc_service| {
        proc_service.borrow().compute_all_concurrent()
    })
}
