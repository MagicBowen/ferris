use super::cost_trait::CostTrait;

pub struct Cpu;

impl Cpu {
    const BASIC_CPU_QUOTA: u32 = 2; /* 基础CPU配额 */
    const CPU_EXCEED_FACTOR: u32 = 10; /* CPU 超出部分的计费因子 */
}

impl CostTrait for Cpu {
    fn cost(&self, usage_time: i32) -> i32 {
        let mut cost = 50;
        if usage_time > Self::BASIC_CPU_QUOTA as i32 {
            let exceed = usage_time as u32 - Self::BASIC_CPU_QUOTA;
            cost += exceed * Self::CPU_EXCEED_FACTOR;
        }
        cost as i32
    }
}
