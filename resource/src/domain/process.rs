use std::hash::Hash;

use crate::domain::allocation::Allocation;

pub type Pid = u32;

pub struct Process {
    pid: Pid,
    allocations: Vec<Allocation>,
}

impl Process {
    pub fn new(pid: Pid) -> Self {
        Process {
            pid,
            allocations: vec![],
        }
    }

    pub fn add_allocation(&mut self, allocation: Allocation) {
        self.allocations.push(allocation);
    }

    pub fn compute_cost(&self) -> i32 {
        self.allocations
            .iter()
            .map(|alloc| alloc.compute_cost())
            .sum::<i32>()
    }

    pub fn compute_penalty(&self) -> i32 {
        self.allocations
            .iter()
            .map(|alloc| alloc.compute_penalty())
            .sum::<i32>()
    }
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}

impl Eq for Process {}

impl Hash for Process {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pid.hash(state);
    }
}
