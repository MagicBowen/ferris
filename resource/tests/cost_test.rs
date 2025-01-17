use resource::*;

#[test]
#[cfg(feature = "platform_high")]
fn compute_cost_and_penalty_of_process_in_high_platform() {
    config_process(0).unwrap();

    config_allocation(0, 3, ResourceType::CPU, 4).unwrap();
    config_allocation(0, 2, ResourceType::Memory, 2048).unwrap();
    config_allocation(0, 14, ResourceType::Storage, 100).unwrap();

    let mut total = 0;
    let mut penalty = 0;
    compute_process(0, &mut total, &mut penalty).unwrap();

    // 手动计算期望值:
    // CPU     : base 50 + (3-2)*10 = 50+10=60
    // MEM     : base 30 + (2 * (2048-1024)*2)=30 + (2*1024*2)=30+4096=4126
    // STORAGE : base 20 + (14-3)*100*1.5=20+(11*100*1.5)=20+1650=1670
    // total = 60 + 4126 + 1670 = 5856
    // STORAGE 资源 >12S, penalty=1
    assert_eq!(total, 5856);
    assert_eq!(penalty, 1);
}

#[test]
#[cfg(feature = "platform_low")]
fn compute_cost_and_penalty_of_process_in_low_platform() {
    config_process(1).unwrap();
    assert!(config_process(1).is_err());

    config_allocation(1, 3, ResourceType::CPU, 4).unwrap();
    config_allocation(1, 2, ResourceType::Memory, 2048).unwrap();

    let mut total = 0;
    let mut penalty = 0;
    compute_process(1, &mut total, &mut penalty).unwrap();

    // 手动计算期望值:
    // CPU     : base 50 + (3-2)*10 = 50+10=60
    // MEM     : base 30 + (2 * (2048-1024)*2)=30 + (2*1024*2)=30+4096=4126
    // total = 60 + 4126 = 4186
    assert_eq!(total, 4186);
    assert_eq!(penalty, 0);
}
