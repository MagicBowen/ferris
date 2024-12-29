#[derive(PartialEq)]
pub enum ResourceType {
    CPU,
    Memory,
    Storage,
}

pub struct Resource {
    pub resource_type: ResourceType,
    pub capacity: i32,
}

impl Resource {
    pub fn new(resource_type: ResourceType, capacity: i32) -> Self {
        Resource { resource_type, capacity }
    }
}