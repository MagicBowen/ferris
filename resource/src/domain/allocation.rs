use crate::domain::resource::Resource;
use crate::domain::resource::{ResourceType, RESOURCE_FACTORY};

pub struct Allocation {
    resource: Box<dyn Resource + Send + Sync>,
    usage_time: i32,
}

impl Allocation {
    pub fn new(resource: Box<dyn Resource + Send + Sync>, usage_time: i32) -> Self {
        Allocation {
            resource,
            usage_time,
        }
    }

    pub(crate) fn compute_cost(&self) -> i32 {
        self.resource.compute_cost(self.usage_time)
    }

    pub(crate) fn compute_penalty(&self) -> i32 {
        self.resource.compute_penalty(self.usage_time)
    }
}

pub struct AllocationFactory;

impl AllocationFactory {
    pub fn create(res: ResourceType, capacity: i32, usage_time: i32) -> Allocation {
        let resource = RESOURCE_FACTORY.create(res, capacity as u32);
        Allocation::new(resource, usage_time)
    }
}
