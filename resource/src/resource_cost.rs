pub trait ResourceCost {
    fn cost(&self, usage_time: i32) -> i32;
    fn penalty(&self, _usage_time: i32) -> i32 {
        0
    }
}
