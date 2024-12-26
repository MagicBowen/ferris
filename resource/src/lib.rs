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

pub fn compute_cost(proc : &Process, total : &mut i32, penalty : &mut i32) {
    for allocation in proc.allocations.iter() {
        let mut cost = 0;
        let exceed;
        match allocation.resource.resource_type {
            ResourceType::CPU => {
                /* 基本cost为50，使用时间大于2s则额外统计 */
                cost += 50;
                if allocation.usage_time > 2 {
                    exceed = allocation.usage_time - 2;
                    cost += exceed * 10;
                }
            }
            ResourceType::Memory => {
                /* 基本cost为30，如果容量超过 BASIC_MEM_QUOTA 则按超出部分和使用时间统计 */
                cost += 30;
                if allocation.resource.capacity > BASIC_MEM_QUOTA {
                    exceed = allocation.resource.capacity - BASIC_MEM_QUOTA;
                    /* 超容部分乘以使用时间 * 2 */
                    cost += allocation.usage_time * exceed * 2;
                }
            }
            ResourceType::Storage => {
                /* 基本cost为20，如果使用时间超过3s，则超出每s统计 1.5 * 容量 */
                cost += 20;
                if allocation.usage_time > 3 {
                    exceed = allocation.usage_time - 3;
                    cost += exceed * allocation.resource.capacity * 3 / 2;
                }
            }
        }

        /* 如果是 IO 资源且使用时间 > 12s，对 penalty 计数加1 */
        if allocation.resource.resource_type == ResourceType::Storage && allocation.usage_time > 12 {
            *penalty += 1;
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
