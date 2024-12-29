pub mod resource;
pub mod allocation;

use allocation::Allocation;

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
    use resource::{Resource, ResourceType};

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
