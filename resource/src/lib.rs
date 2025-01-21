mod domain;
mod service;

use cfg_if::cfg_if;

pub use domain::resource::ResourceType;
use service::ProcService;
use service::ProcessRepo;

cfg_if! {
    if #[cfg(feature = "singleton_local_thread")] {
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
    }
    else if #[cfg(feature = "singleton_lazy_static")] {
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
    } else {

        use std::sync::OnceLock;

        static PROCESS_REPO: OnceLock<ProcessRepo> = OnceLock::new();
        static PROC_SERVICE: OnceLock<ProcService<'static>> = OnceLock::new();
        
        fn get_process_repo() -> &'static ProcessRepo {
            PROCESS_REPO.get_or_init(|| ProcessRepo::new())
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
        
        pub fn clear_all() {
            let repo = get_process_repo();
            repo.clear();
        }
    }
}