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
            .borrow_mut()
            .add_allocation(pid, time, res, capacity)
    })
}

pub fn compute_process(pid: u32, cost: &mut i32, penalty: &mut i32) -> Result<(), String> {
    PROC_SERVICE.with(|proc_service| {
        proc_service.borrow_mut().compute_process(pid).map(|(c, p)| {
            *cost += c;
            *penalty += p;
        })
    })
}
