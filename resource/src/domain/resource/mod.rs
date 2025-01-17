mod factory;

use once_cell::sync::Lazy;
use factory::ResourceFactory;

pub trait Resource {
    fn compute_cost(&self, usage_time: i32) -> i32;
    fn compute_penalty(&self, _: i32) -> i32 {
        0
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

pub static RESOURCE_FACTORY: Lazy<ResourceFactory> = Lazy::new(|| {
    let mut factory = ResourceFactory::new();
    #[cfg(feature = "resource_cpu")]
    factory.register(ResourceType::CPU, |_: u32| Box::new(cpu::Cpu));
    #[cfg(feature = "resource_memory")]
    factory.register(ResourceType::Memory, |capacity: u32| {
        Box::new(memory::Memory::new(capacity))
    });
    #[cfg(feature = "resource_storage")]
    factory.register(ResourceType::Storage, |capacity: u32| {
        Box::new(storage::Storage::new(capacity))
    });
    factory
});


// use std::sync::OnceLock;

// struct ResourceFactoryInstance {
//     factory: ResourceFactory,
// }

// static RESOURCE_FACTORY: OnceLock<ResourceFactoryInstance> = OnceLock::new();

// impl ResourceFactoryInstance {
//     pub fn get() -> &'static ResourceFactory {
//         RESOURCE_FACTORY.get_or_init(|| ResourceFactoryInstance {
//             factory: {
//                 let mut factory = ResourceFactory::new();
//                 #[cfg(feature = "resource_cpu")]
//                 factory.register(ResourceType::CPU, |_: u32| Box::new(cpu::Cpu));
//                 #[cfg(feature = "resource_memory")]
//                 factory.register(ResourceType::Memory, |capacity: u32| {
//                     Box::new(memory::Memory::new(capacity))
//                 });
//                 #[cfg(feature = "resource_storage")]
//                 factory.register(ResourceType::Storage, |capacity: u32| {
//                     Box::new(storage::Storage::new(capacity))
//                 });
//                 factory
//             },
//         });
//         &RESOURCE_FACTORY.get().unwrap().factory
//     }
// }
