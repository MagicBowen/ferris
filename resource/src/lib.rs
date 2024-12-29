#[derive(PartialEq)]
pub enum ResourceType {
    CPU,
    Memory,
    Storage,
}

pub struct Resource {
    resource_type: ResourceType,
    capacity: i32,
}

pub struct Allocation {
    resource: Resource,
    usage_time: i32,
}

pub struct Process {
    allocations: Vec<Allocation>,
}

const BASIC_MEM_QUOTA: i32 = 1024; /* 基础内存配额 */
const BASIC_CPU_QUOTA: i32 = 2;    /* 基础CPU配额 */
const BASIC_STORAGE_QUOTA: i32 = 3; /* 基础存储配额 */

const CPU_EXCEED_FACTOR: i32 = 10; /* CPU 超出部分的计费因子 */
const MEM_EXCEED_FACTOR: i32 = 2;  /* MEM 超出部分的计费因子 */
const STORAGE_EXCEED_FACTOR: f32 = 1.5; /* STORAGE 超出部分的计费因子 */

const STORAGE_PENALTY_THRESHOLD: i32 = 12; /* STORAGE 超出部分的计费因子 */
const STORAGE_PENALTY: i32 = 1; /* STORAGE 超出部分的计费因子 */

pub fn compute_cost(proc : &Process, total : &mut i32, penalty : &mut i32) {
    for allocation in proc.allocations.iter() {
        let mut cost = 0;
        match allocation.resource.resource_type {
            ResourceType::CPU => {
                cost += 50;
                if allocation.usage_time > BASIC_CPU_QUOTA {
                    let exceed = allocation.usage_time - BASIC_CPU_QUOTA;
                    cost += exceed * CPU_EXCEED_FACTOR;
                }
            }
            ResourceType::Memory => {
                cost += 30;
                if allocation.resource.capacity > BASIC_MEM_QUOTA {
                    let exceed = allocation.resource.capacity - BASIC_MEM_QUOTA;
                    cost += allocation.usage_time * exceed * MEM_EXCEED_FACTOR;
                }
            }
            ResourceType::Storage => {
                cost += 20;
                if allocation.usage_time > BASIC_STORAGE_QUOTA {
                    let exceed = allocation.usage_time - BASIC_STORAGE_QUOTA;
                    cost += ((exceed * allocation.resource.capacity) as f32 * STORAGE_EXCEED_FACTOR) as i32;
                }
            }
        }

        if allocation.resource.resource_type == ResourceType::Storage && allocation.usage_time > STORAGE_PENALTY_THRESHOLD {
            *penalty += STORAGE_PENALTY;
        }

        *total += cost;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_cost_and_penalty_of_process() {
        let r1 = Resource { resource_type: ResourceType::CPU, capacity: 4 };
        let r2 = Resource { resource_type: ResourceType::Memory, capacity: 2048 };
        let r3 = Resource { resource_type: ResourceType::Storage, capacity: 100 };
    
        let a1 = Allocation { resource: r1, usage_time: 3 };  // CPU     使用3s
        let a2 = Allocation { resource: r2, usage_time: 2 };  // Memory  使用2s
        let a3 = Allocation { resource: r3, usage_time: 14 }; // Storage 使用14s
    
        let proc = Process { allocations: vec![a1, a2, a3] };
    
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
}
