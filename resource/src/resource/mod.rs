mod cpu;
mod memory;
mod storage;
mod factory;

use crate::resource_cost::ResourceCost;
use factory::RES_FACTORY;

#[derive(Debug, PartialEq, Eq, Hash)]
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
        let resource_cost = RES_FACTORY.lock().unwrap().create(resource_type, capacity);
        Resource {resource_cost,}
    }

    pub fn compute_cost(&self, usage_time : i32) -> i32 {
        self.resource_cost.cost(usage_time)
    }

    pub fn compute_penalty(&self, usage_time : i32) -> i32 {
        self.resource_cost.penalty(usage_time)
    }
}

pub fn register() {
    cpu::register_resource();
    memory::register_resource();
    storage::register_resource();
}