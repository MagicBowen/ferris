use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "singleton_local_thread")] {
        mod thread_local_app;
        pub use thread_local_app::{config_process, config_allocation, compute_process, compute_all, compute_all_concurrent};
    }
    else if #[cfg(feature = "singleton_lazy_static")] {
        mod lazy_static_app;
        pub use lazy_static_app::{config_process, config_allocation, compute_process, compute_all, compute_all_concurrent};
    } else {
        mod once_lock_app;
        pub use once_lock_app::{config_process, config_allocation, compute_process, compute_all, compute_all_concurrent};
    }
}