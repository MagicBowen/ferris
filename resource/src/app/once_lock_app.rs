use crate::service::ProcService;
use crate::service::ProcessRepo;
use crate::ResourceType;

use std::sync::OnceLock;

static PROCESS_REPO: OnceLock<ProcessRepo> = OnceLock::new();
static PROC_SERVICE: OnceLock<ProcService<'static>> = OnceLock::new();

fn get_process_repo() -> &'static ProcessRepo {
    PROCESS_REPO.get_or_init(ProcessRepo::new)
}

fn get_proc_service() -> &'static ProcService<'static> {
    PROC_SERVICE.get_or_init(|| ProcService::new(get_process_repo()))
}

pub fn config_process(pid: u32) -> Result<(), String> {
    get_proc_service().add_process(pid)
}

pub fn config_allocation(
    pid: u32,
    time: i32,
    res: ResourceType,
    capacity: i32,
) -> Result<(), String> {
    get_proc_service().add_allocation(pid, time, res, capacity)
}

pub fn compute_process(pid: u32) -> Option<(i32, i32)> {
    get_proc_service().compute_process(pid)
}

pub fn compute_all() -> Vec<(u32, i32, i32)> {
    get_proc_service().compute_all()
}

pub fn compute_all_concurrent() -> Vec<(u32, i32, i32)> {
    get_proc_service().compute_all_concurrent()
}
