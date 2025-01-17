use crate::domain::resource::Resource;

pub struct Allocation {
    resource: Resource,
    usage_time: i32,
}

impl Allocation {
    pub fn new(resource: Resource, usage_time: i32) -> Self {
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
