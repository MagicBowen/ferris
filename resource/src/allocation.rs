use crate::resource::{Resource, ResourceType};

const BASIC_MEM_QUOTA: i32 = 1024; /* 基础内存配额 */
const BASIC_CPU_QUOTA: i32 = 2;    /* 基础CPU配额 */
const BASIC_STORAGE_QUOTA: i32 = 3; /* 基础存储配额 */

const CPU_EXCEED_FACTOR: i32 = 10; /* CPU 超出部分的计费因子 */
const MEM_EXCEED_FACTOR: i32 = 2;  /* MEM 超出部分的计费因子 */
const STORAGE_EXCEED_FACTOR: f32 = 1.5; /* STORAGE 超出部分的计费因子 */

const STORAGE_PENALTY_THRESHOLD: i32 = 12; /* STORAGE 超出部分的计费因子 */
const STORAGE_PENALTY: i32 = 1; /* STORAGE 超出部分的计费因子 */

pub struct Allocation {
    pub resource: Resource,
    pub usage_time: i32,
}

impl Allocation {
    #[cfg(test)]
    pub fn new(resource: Resource, usage_time: i32) -> Self {
        Allocation { resource, usage_time }
    }

    pub fn compute_cost(&self) -> i32 {
        let mut cost = 0;
        match self.resource.resource_type {
            ResourceType::CPU => {
                cost += 50;
                if self.usage_time > BASIC_CPU_QUOTA {
                    let exceed = self.usage_time - BASIC_CPU_QUOTA;
                    cost += exceed * CPU_EXCEED_FACTOR;
                }
            }
            ResourceType::Memory => {
                cost += 30;
                if self.resource.capacity > BASIC_MEM_QUOTA {
                    let exceed = self.resource.capacity - BASIC_MEM_QUOTA;
                    cost += self.usage_time * exceed * MEM_EXCEED_FACTOR;
                }
            }
            ResourceType::Storage => {
                cost += 20;
                if self.usage_time > BASIC_STORAGE_QUOTA {
                    let exceed = self.usage_time - BASIC_STORAGE_QUOTA;
                    cost += ((exceed * self.resource.capacity) as f32 * STORAGE_EXCEED_FACTOR) as i32;
                }
            }
        }

        cost
    }

    pub fn compute_penalty(&self) -> i32 {
        if self.resource.resource_type == ResourceType::Storage && self.usage_time > STORAGE_PENALTY_THRESHOLD {
            STORAGE_PENALTY
        } else {
            0
        }
    }
}