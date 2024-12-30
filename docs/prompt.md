下面的代码帮我改写为多态，将 Resource 中的 compute_cost 和 compute_penalty 委托给具体不同的 Resource 中；

```rust
pub enum ResourceType {
    CPU,
    Memory,
    Storage,
}

pub struct Resource {
    resource_type: ResourceType,
    capacity: i32,
}

const BASIC_MEM_QUOTA: i32 = 1024; /* 基础内存配额 */
const BASIC_CPU_QUOTA: i32 = 2;    /* 基础CPU配额 */
const BASIC_STORAGE_QUOTA: i32 = 3; /* 基础存储配额 */

const CPU_EXCEED_FACTOR: i32 = 10; /* CPU 超出部分的计费因子 */
const MEM_EXCEED_FACTOR: i32 = 2;  /* MEM 超出部分的计费因子 */
const STORAGE_EXCEED_FACTOR: f32 = 1.5; /* STORAGE 超出部分的计费因子 */

const STORAGE_PENALTY_THRESHOLD: i32 = 12; /* STORAGE 超出部分的计费因子 */
const STORAGE_PENALTY: i32 = 1; /* STORAGE 超出部分的计费因子 */ 

impl Resource {

    pub fn new(resource_type: ResourceType, capacity: i32) -> Self {
        Resource { resource_type, capacity }
    }

    pub fn compute_cost(&self, usage_time : i32) -> i32 {
        let mut cost = 0;
        match self.resource_type {
            ResourceType::CPU => {
                cost += 50;
                if usage_time > BASIC_CPU_QUOTA {
                    let exceed = usage_time - BASIC_CPU_QUOTA;
                    cost += exceed * CPU_EXCEED_FACTOR;
                }
            }
            ResourceType::Memory => {
                cost += 30;
                if self.capacity > BASIC_MEM_QUOTA {
                    let exceed = self.capacity - BASIC_MEM_QUOTA;
                    cost += usage_time * exceed * MEM_EXCEED_FACTOR;
                }
            }
            ResourceType::Storage => {
                cost += 20;
                if usage_time > BASIC_STORAGE_QUOTA {
                    let exceed = usage_time - BASIC_STORAGE_QUOTA;
                    cost += ((exceed * self.capacity) as f32 * STORAGE_EXCEED_FACTOR) as i32;
                }
            }
        }

        cost
    }

    pub fn compute_penalty(&self, usage_time : i32) -> i32 {
        if self.resource_type == ResourceType::Storage && usage_time > STORAGE_PENALTY_THRESHOLD {
            STORAGE_PENALTY
        } else {
            0
        }
    }
}
```