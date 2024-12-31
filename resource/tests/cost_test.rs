use resource::resource::{Resource, ResourceType};
use resource::allocation::Allocation;
use resource::process::Process;
use resource::compute_cost;

#[test]
#[cfg(feature = "platform_high")]
fn compute_cost_and_penalty_of_process_in_high_platform() {
    resource::resource::register();

    let r1 = Resource::new(ResourceType::CPU, 4);
    let r2 = Resource::new(ResourceType::Memory, 2048);
    let r3 = Resource::new(ResourceType::Storage, 100);

    let a1 = Allocation::new(r1, 3);
    let a2 = Allocation::new(r2, 2);
    let a3 = Allocation::new(r3, 14);

    let mut proc : Process = Process::new();
    proc.add_allocation(a1);
    proc.add_allocation(a2);
    proc.add_allocation(a3);

    let mut total = 0;
    let mut penalty = 0;
    compute_cost(&proc, &mut total, &mut penalty);

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
    resource::resource::register();

    let r1 = Resource::new(ResourceType::CPU, 4);
    let r2 = Resource::new(ResourceType::Memory, 2048);

    let a1 = Allocation::new(r1, 3);
    let a2 = Allocation::new(r2, 2);

    let mut proc : Process = Process::new();
    proc.add_allocation(a1);
    proc.add_allocation(a2);

    let mut total = 0;
    let mut penalty = 0;
    compute_cost(&proc, &mut total, &mut penalty);

    // 手动计算期望值:
    // CPU     : base 50 + (3-2)*10 = 50+10=60
    // MEM     : base 30 + (2 * (2048-1024)*2)=30 + (2*1024*2)=30+4096=4126
    // total = 60 + 4126 = 4186
    assert_eq!(total, 4186);
    assert_eq!(penalty, 0);
}