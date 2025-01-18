use resource::*;

#[test]
#[cfg(feature = "platform_high")]
fn compute_cost_and_penalty_of_process_in_high_platform() {
    config_process(0).unwrap();

    config_allocation(0, 3, ResourceType::CPU, 4).unwrap();
    config_allocation(0, 2, ResourceType::Memory, 2048).unwrap();
    config_allocation(0, 14, ResourceType::Storage, 100).unwrap();

    assert_eq!(compute_process(0), Some((5856, 1)));
}

#[test]
#[cfg(feature = "platform_low")]
fn compute_cost_and_penalty_of_process_in_low_platform() {
    config_process(1).unwrap();
    assert!(config_process(1).is_err());

    config_allocation(1, 3, ResourceType::CPU, 4).unwrap();
    config_allocation(1, 2, ResourceType::Memory, 2048).unwrap();

    assert_eq!(compute_process(1), Some((4186, 0)));
}

#[test]
#[cfg(feature = "platform_high")]
fn compute_cost_and_penalty_of_all_processes() {
    config_process(0).unwrap();
    config_process(1).unwrap();

    config_allocation(0, 3, ResourceType::CPU, 4).unwrap();
    config_allocation(0, 2, ResourceType::Memory, 2048).unwrap();
    config_allocation(0, 14, ResourceType::Storage, 100).unwrap();

    config_allocation(1, 3, ResourceType::CPU, 4).unwrap();
    config_allocation(1, 2, ResourceType::Memory, 2048).unwrap();

    let result = compute_all();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&(0, 5856, 1)));
    assert!(result.contains(&(1, 4186, 0)));
}

#[test]
#[cfg(feature = "platform_high")]
fn compute_cost_and_penalty_of_all_processes_concurrency() {
    config_process(0).unwrap();
    config_process(1).unwrap();

    config_allocation(0, 3, ResourceType::CPU, 4).unwrap();
    config_allocation(0, 2, ResourceType::Memory, 2048).unwrap();
    config_allocation(0, 14, ResourceType::Storage, 100).unwrap();

    config_allocation(1, 3, ResourceType::CPU, 4).unwrap();
    config_allocation(1, 2, ResourceType::Memory, 2048).unwrap();

    let result = compute_all_concurrent();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&(0, 5856, 1)));
    assert!(result.contains(&(1, 4186, 0)));
}