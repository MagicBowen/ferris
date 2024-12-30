pub trait ResourceCost {
    fn cost(&self, usage_time : i32) -> i32;
    fn penalty(&self, _usage_time : i32) -> i32 {
        0
    }
}

pub struct CpuResource;

impl CpuResource {
    const BASIC_CPU_QUOTA: u32 = 2;    /* 基础CPU配额 */
    const CPU_EXCEED_FACTOR: u32 = 10; /* CPU 超出部分的计费因子 */
}

impl ResourceCost for CpuResource {
    fn cost(&self, usage_time : i32) -> i32 {
        let mut cost = 50;
        if usage_time > Self::BASIC_CPU_QUOTA as i32 {
            let exceed = usage_time as u32 - Self::BASIC_CPU_QUOTA;
            cost += exceed * Self::CPU_EXCEED_FACTOR;
        }
        cost as i32
    }
}

pub struct MemoryResource {
    mb_size : u32,
}

impl MemoryResource {
    pub fn new(mb_size: u32) -> Self {
        MemoryResource { mb_size }
    }

    const BASIC_MEM_QUOTA: u32 = 1024; /* 基础内存配额 */
    const MEM_EXCEED_FACTOR: u32 = 2;  /* MEM 超出部分的计费因子 */
}

impl ResourceCost for MemoryResource {
    fn cost(&self, usage_time : i32) -> i32 {
        let mut cost = 30;
        if self.mb_size > Self::BASIC_MEM_QUOTA {
            let exceed = self.mb_size - Self::BASIC_MEM_QUOTA;
            cost += usage_time as u32 * exceed * Self::MEM_EXCEED_FACTOR;
        }
        cost as i32
    }
}

pub struct StorageResource {
    gb_size : u32,
}

impl StorageResource {
    pub fn new(gb_size: u32) -> Self {
        StorageResource { gb_size }
    }

    const BASIC_STORAGE_QUOTA: i32 = 3; /* 基础存储配额 */
    const STORAGE_EXCEED_FACTOR: f32 = 1.5; /* STORAGE 超出部分的计费因子 */
    const STORAGE_PENALTY_THRESHOLD: u32 = 12; /* STORAGE 超出部分的计费因子 */
    const STORAGE_PENALTY: u32 = 1; /* STORAGE 超出部分的计费因子 */ 
}

impl ResourceCost for StorageResource {
    fn cost(&self, usage_time : i32) -> i32 {
        let mut cost = 20;
        if usage_time > Self::BASIC_STORAGE_QUOTA as i32 {
            let exceed = (usage_time - Self::BASIC_STORAGE_QUOTA) as u32;
            cost += ((exceed * self.gb_size) as f32 * Self::STORAGE_EXCEED_FACTOR) as i32;
        }
        cost
    }

    fn penalty(&self, usage_time : i32) -> i32 {
        if usage_time > Self::STORAGE_PENALTY_THRESHOLD as i32 {
            Self::STORAGE_PENALTY as i32
        } else {
            0
        }
    }
}