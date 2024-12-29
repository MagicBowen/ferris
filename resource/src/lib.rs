#[derive(PartialEq)]
pub enum ResourceType {
    CPU,
    Memory,
    Storage,
}


const BASIC_MEM_QUOTA: i32 = 1024; /* 基础内存配额 */
const BASIC_CPU_QUOTA: i32 = 2;    /* 基础CPU配额 */
const BASIC_STORAGE_QUOTA: i32 = 3; /* 基础存储配额 */

const CPU_EXCEED_FACTOR: i32 = 10; /* CPU 超出部分的计费因子 */
const MEM_EXCEED_FACTOR: i32 = 2;  /* MEM 超出部分的计费因子 */
const STORAGE_EXCEED_FACTOR: f32 = 1.5; /* STORAGE 超出部分的计费因子 */

const STORAGE_PENALTY_THRESHOLD: i32 = 12; /* STORAGE 超出部分的计费因子 */
const STORAGE_PENALTY: i32 = 1; /* STORAGE 超出部分的计费因子 */

pub struct Resource {
    resource_type: ResourceType,
    capacity: i32,
}

impl Resource {
    #[cfg(test)]
    fn new(resource_type: ResourceType, capacity: i32) -> Self {
        Resource { resource_type, capacity }
    }
}

pub struct Allocation {
    resource: Resource,
    usage_time: i32,
}

impl Allocation {
    #[cfg(test)]
    fn new(resource: Resource, usage_time: i32) -> Self {
        Allocation { resource, usage_time }
    }

    fn compute_cost(&self) -> i32 {
        let mut cost = 0;
        match self.resource.resource_type {
            ResourceType::CPU => {
                cost += 50;
                if self.usage_time > BASIC_CPU_QUOTA {
                    let exceed = self.usage_time - BASIC_CPU_QUOTA;
                    cost += exceed * CPU_EXCEED_FACTOR;
                }
            }
            ResourceType::Memory => {
                cost += 30;
                if self.resource.capacity > BASIC_MEM_QUOTA {
                    let exceed = self.resource.capacity - BASIC_MEM_QUOTA;
                    cost += self.usage_time * exceed * MEM_EXCEED_FACTOR;
                }
            }
            ResourceType::Storage => {
                cost += 20;
                if self.usage_time > BASIC_STORAGE_QUOTA {
                    let exceed = self.usage_time - BASIC_STORAGE_QUOTA;
                    cost += ((exceed * self.resource.capacity) as f32 * STORAGE_EXCEED_FACTOR) as i32;
                }
            }
        }

        cost
    }

    fn compute_penalty(&self) -> i32 {
        if self.resource.resource_type == ResourceType::Storage && self.usage_time > STORAGE_PENALTY_THRESHOLD {
            STORAGE_PENALTY
        } else {
            0
        }
    }
}

pub struct Process {
    allocations: Vec<Allocation>,
}

impl Process {
    #[cfg(test)]
    fn new() -> Self {
        Process { allocations: vec![] }
    }

    #[cfg(test)]
    fn add_allocation(&mut self, allocation: Allocation) {
        self.allocations.push(allocation);
    }

    pub fn compute_cost(&self, total : &mut i32, penalty : &mut i32) {
        for allocation in &self.allocations {
            *penalty += allocation.compute_penalty();
            *total += allocation.compute_cost();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_cost_and_penalty_of_process() {
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
        proc.compute_cost(&mut total, &mut penalty);
    
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
