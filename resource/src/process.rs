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
        for allocation in &self.allocations {
            *cost += allocation.compute_cost();
        }
    }

    pub fn compute_penalty(&self, penalty : &mut i32) {
        for allocation in &self.allocations {
            *penalty += allocation.compute_penalty();
        }
    }
}
