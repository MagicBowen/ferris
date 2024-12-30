pub mod cpu;
pub mod memory;
pub mod storage;

use crate::resource_cost::ResourceCost;
use cpu::Cpu;
use memory::Memory;
use storage::Storage;

#[derive(PartialEq)]
pub enum ResourceType {
    CPU,
    Memory,
    Storage,
}

pub struct Resource {
    resource_cost: Box<dyn ResourceCost>,
}

impl Resource {
    pub fn new(resource_type: ResourceType, capacity: i32) -> Self {
        let resource_cost :Box<dyn ResourceCost> = match resource_type {
            ResourceType::CPU => Box::new(Cpu{}),
            ResourceType::Memory => Box::new(Memory::new(capacity as u32)),
            ResourceType::Storage => Box::new(Storage::new(capacity as u32)),
        };

        Resource {resource_cost,}
    }

    pub fn compute_cost(&self, usage_time : i32) -> i32 {
        self.resource_cost.cost(usage_time)
    }

    pub fn compute_penalty(&self, usage_time : i32) -> i32 {
        self.resource_cost.penalty(usage_time)
    }
}