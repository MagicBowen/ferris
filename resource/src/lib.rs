mod domain;
mod service;
mod app;

pub use domain::resource::ResourceType;
pub use app::{config_process, config_allocation, compute_process, compute_all, compute_all_concurrent};