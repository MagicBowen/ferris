use super::Resource;
use super::ResourceType;
use std::collections::HashMap;

type FactoryFn = fn(capacity: u32) -> Box<dyn Resource + Send + Sync>;
pub struct ResourceFactory {
    registry: HashMap<ResourceType, FactoryFn>,
}

impl ResourceFactory {
    pub fn new() -> Self {
        ResourceFactory {
            registry: HashMap::new(),
        }
    }

    pub fn register(&mut self, resource_type: ResourceType, factory: FactoryFn) {
        self.registry.insert(resource_type, factory);
    }

    pub fn create(
        &self,
        resource_type: ResourceType,
        capacity: u32,
    ) -> Box<dyn Resource + Send + Sync> {
        if let Some(factory) = self.registry.get(&resource_type) {
            factory(capacity)
        } else {
            panic!("Resource type {:?} not registered", resource_type);
        }
    }
}
