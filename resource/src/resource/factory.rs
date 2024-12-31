use crate::resource_cost::ResourceCost;
use super::ResourceType;
use std::collections::HashMap;
use std::sync::Mutex;

type FactoryFn = fn(capacity: i32) -> Box<dyn ResourceCost>;
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

    pub fn create(&self, resource_type: ResourceType, capacity: i32) -> Box<dyn ResourceCost> {
        if let Some(factory) = self.registry.get(&resource_type) {
            factory(capacity)
        } else {
            panic!("Resource type {:?} not registered", resource_type);
        }
    }
}

lazy_static::lazy_static! {
    pub static ref RES_FACTORY: Mutex<ResourceFactory> = Mutex::new(ResourceFactory::new());
}