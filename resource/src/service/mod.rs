pub mod repo;
pub mod proc_service;

pub use repo::ProcessRepo;
pub use proc_service::ProcService;

#[cfg(test)]

use super::*;

#[test]
#[cfg(feature = "platform_low")]
fn compute_cost_and_penalty_of_process_in_low_platform() {
    let repo = ProcessRepo::new();
    let service = ProcService::new(&repo);

    service.add_process(1).unwrap();
    assert!(service.add_process(1).is_err());

    service.add_allocation(1, 3, ResourceType::CPU, 4).unwrap();
    service.add_allocation(1, 2, ResourceType::Memory, 2048).unwrap();

    assert_eq!(service.compute_process(1), Some((4186, 0)));
}

#[test]
#[cfg(feature = "platform_high")]
fn compute_cost_and_penalty_of_process_in_high_platform() {
    let repo = ProcessRepo::new();
    let service = ProcService::new(&repo);

    service.add_process(0).unwrap();

    service.add_allocation(0, 3, ResourceType::CPU, 4).unwrap();
    service.add_allocation(0, 2, ResourceType::Memory, 2048).unwrap();
    service.add_allocation(0, 14, ResourceType::Storage, 100).unwrap();

    assert_eq!(service.compute_process(0), Some((5856, 1)));
}

#[test]
#[cfg(feature = "platform_high")]
fn compute_cost_and_penalty_of_all_processes() {
    let repo = ProcessRepo::new();
    let service = ProcService::new(&repo);

    service.add_process(0).unwrap();
    service.add_process(1).unwrap();

    service.add_allocation(0, 3, ResourceType::CPU, 4).unwrap();
    service.add_allocation(0, 2, ResourceType::Memory, 2048).unwrap();
    service.add_allocation(0, 14, ResourceType::Storage, 100).unwrap();

    service.add_allocation(1, 3, ResourceType::CPU, 4).unwrap();
    service.add_allocation(1, 2, ResourceType::Memory, 2048).unwrap();

    let result = service.compute_all();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&(0, 5856, 1)));
    assert!(result.contains(&(1, 4186, 0)));
}

#[test]
#[cfg(feature = "platform_high")]
fn compute_cost_and_penalty_of_all_processes_concurrency() {
    let repo = ProcessRepo::new();
    let service = ProcService::new(&repo);

    service.add_process(0).unwrap();
    service.add_process(1).unwrap();

    service.add_allocation(0, 3, ResourceType::CPU, 4).unwrap();
    service.add_allocation(0, 2, ResourceType::Memory, 2048).unwrap();
    service.add_allocation(0, 14, ResourceType::Storage, 100).unwrap();

    service.add_allocation(1, 3, ResourceType::CPU, 4).unwrap();
    service.add_allocation(1, 2, ResourceType::Memory, 2048).unwrap();

    let result = service.compute_all_concurrent();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&(0, 5856, 1)));
    assert!(result.contains(&(1, 4186, 0)));
}
