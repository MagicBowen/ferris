use super::Resource;
use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct MB(u32);

impl MB {
    const fn new(value: u32) -> Self {
        MB(value)
    }

    const fn as_u32(&self) -> u32 {
        self.0
    }
}

impl Add for MB {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        MB(self.0 + other.0)
    }
}

impl Sub for MB {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        MB(self.0 - other.0)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Memory {
    mb_size: MB,
}

impl Memory {
    pub fn new(mb_size: u32) -> Self {
        Memory {
            mb_size: MB::new(mb_size),
        }
    }

    const BASIC_MEM_QUOTA: MB = MB::new(1024);
    const MEM_EXCEED_FACTOR: u32 = 2;
}

impl Resource for Memory {
    fn compute_cost(&self, usage_time: i32) -> i32 {
        let mut cost = 30;
        if self.mb_size > Self::BASIC_MEM_QUOTA {
            let exceed = self.mb_size - Self::BASIC_MEM_QUOTA;
            cost += usage_time as u32 * exceed.as_u32() * Self::MEM_EXCEED_FACTOR;
        }
        cost as i32
    }
}
