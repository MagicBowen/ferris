use super::ResourceCost;
pub struct Memory {
    mb_size : u32,
}

impl Memory {
    pub fn new(mb_size: u32) -> Self {
        Memory { mb_size }
    }

    const BASIC_MEM_QUOTA: u32 = 1024; /* 基础内存配额 */
    const MEM_EXCEED_FACTOR: u32 = 2;  /* MEM 超出部分的计费因子 */
}

impl ResourceCost for Memory {
    fn cost(&self, usage_time : i32) -> i32 {
        let mut cost = 30;
        if self.mb_size > Self::BASIC_MEM_QUOTA {
            let exceed = self.mb_size - Self::BASIC_MEM_QUOTA;
            cost += usage_time as u32 * exceed * Self::MEM_EXCEED_FACTOR;
        }
        cost as i32
    }
}