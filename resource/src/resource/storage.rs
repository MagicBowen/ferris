use super::ResourceCost;

pub struct Storage {
    gb_size : u32,
}

impl Storage {
    pub fn new(gb_size: u32) -> Self {
        Storage { gb_size }
    }

    const BASIC_STORAGE_QUOTA: i32 = 3; /* 基础存储配额 */
    const STORAGE_EXCEED_FACTOR: f32 = 1.5; /* STORAGE 超出部分的计费因子 */
    const STORAGE_PENALTY_THRESHOLD: u32 = 12; /* STORAGE 超出部分的计费因子 */
    const STORAGE_PENALTY: u32 = 1; /* STORAGE 超出部分的计费因子 */ 
}

impl ResourceCost for Storage {
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