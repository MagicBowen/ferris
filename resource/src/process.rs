use crate::allocation::Allocation;

pub struct Process {
    pub allocations: Vec<Allocation>,
}

impl Process {
    pub fn new() -> Self {
        Process { allocations: vec![] }
    }

    pub fn add_allocation(&mut self, allocation: Allocation) {
        self.allocations.push(allocation);
    }

    pub fn compute_cost(&self, total : &mut i32) {
        for allocation in &self.allocations {
            *total += allocation.compute_cost();
        }
    }

    pub fn compute_penalty(&self, penalty : &mut i32) {
        for allocation in &self.allocations {
            *penalty += allocation.compute_penalty();
        }
    }
}
