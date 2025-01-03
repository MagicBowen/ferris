pub mod allocation;
pub mod process;
pub mod resource;

pub fn compute_cost(proc: &process::Process, cost: &mut i32, penalty: &mut i32) {
    proc.compute_cost(cost);
    proc.compute_penalty(penalty);
}
