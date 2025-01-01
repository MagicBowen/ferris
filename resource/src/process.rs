use crate::allocation::Allocation;

pub struct Process {
    allocations: Vec<Allocation>,
}

impl Process {
    pub fn new() -> Self {
        Process { allocations: vec![] }
    }

    pub fn add_allocation(&mut self, allocation: Allocation) {
        self.allocations.push(allocation);
    }

    pub fn compute_cost(&self, cost : &mut i32) {
        *cost += self.allocations.iter().map(|alloc| alloc.compute_cost()).sum::<i32>();
    }

    pub fn compute_penalty(&self, penalty : &mut i32) {
        *penalty += self.allocations.iter().map(|alloc| alloc.compute_penalty()).sum::<i32>();
    }
}
