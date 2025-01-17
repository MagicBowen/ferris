use super::Resource;
use std::ops::{Add, Sub};

#[derive(PartialEq, PartialOrd, Clone, Copy)]
struct GB(u32);

impl GB {
    const fn new(value: u32) -> Self {
        GB(value)
    }

    const fn as_u32(&self) -> u32 {
        self.0
    }
}

impl Add for GB {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        GB(self.0 + other.0)
    }
}

impl Sub for GB {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        GB(self.0 - other.0)
    }
}

pub struct Storage {
    gb_size: GB,
}

impl Storage {
    pub fn new(gb_size: u32) -> Self {
        Storage {
            gb_size: GB::new(gb_size),
        }
    }

    const BASIC_STORAGE_QUOTA: i32 = 3; /* 基础存储配额 */
    const STORAGE_EXCEED_FACTOR: f32 = 1.5; /* STORAGE 超出部分的计费因子 */
    const STORAGE_PENALTY_THRESHOLD: u32 = 12; /* STORAGE 超出部分的计费因子 */
    const STORAGE_PENALTY: u32 = 1; /* STORAGE 超出部分的计费因子 */
}

impl Resource for Storage {
    fn compute_cost(&self, usage_time: i32) -> i32 {
        let mut cost = 20;
        if usage_time > Self::BASIC_STORAGE_QUOTA as i32 {
            let exceed = (usage_time - Self::BASIC_STORAGE_QUOTA) as u32;
            cost += ((exceed * self.gb_size.as_u32()) as f32 * Self::STORAGE_EXCEED_FACTOR) as i32;
        }
        cost
    }

    fn compute_penalty(&self, usage_time: i32) -> i32 {
        if usage_time > Self::STORAGE_PENALTY_THRESHOLD as i32 {
            Self::STORAGE_PENALTY as i32
        } else {
            0
        }
    }
}
