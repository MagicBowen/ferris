mod factory;

use crate::resource_cost::ResourceCost;
use factory::ResourceFactory;
use std::sync::Mutex;

pub struct Resource {
    resource_cost: Box<dyn ResourceCost>,
}

impl Resource {
    pub fn new(resource_type: ResourceType, capacity: i32) -> Self {
        let resource_cost = RES_FACTORY.lock().unwrap().create(resource_type, capacity as u32);
        Resource {resource_cost,}
    }

    pub fn compute_cost(&self, usage_time : i32) -> i32 {
        self.resource_cost.cost(usage_time)
    }

    pub fn compute_penalty(&self, usage_time : i32) -> i32 {
        self.resource_cost.penalty(usage_time)
    }
}

//////////////////////////////////////////////////////////////////
#[cfg(feature = "resource_cpu")]
mod cpu;

#[cfg(feature = "resource_memory")]
mod memory;

#[cfg(feature = "resource_storage")]
mod storage;


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ResourceType {
    #[cfg(feature = "resource_cpu")]
    CPU,
    #[cfg(feature = "resource_memory")]
    Memory,
    #[cfg(feature = "resource_storage")]
    Storage,
}

lazy_static::lazy_static! {
    pub static ref RES_FACTORY: Mutex<ResourceFactory> = Mutex::new({
        let mut factory = ResourceFactory::new();
        #[cfg(feature = "resource_cpu")]
        factory.register(ResourceType::CPU, |_: u32| Box::new(cpu::Cpu));
        #[cfg(feature = "resource_memory")]
        factory.register(ResourceType::Memory, |capacity: u32| Box::new(memory::Memory::new(capacity)));
        #[cfg(feature = "resource_storage")]
        factory.register(ResourceType::Storage, |capacity: u32| Box::new(storage::Storage::new(capacity)));
        factory
    });
}
//////////////////////////////////////////////////////////////////
