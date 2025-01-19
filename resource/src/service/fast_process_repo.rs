// use crate::domain::process::{Pid, Process};
// use dashmap::DashMap;
// use rayon::prelude::*;
// use std::sync::{Arc, Mutex};

// pub struct ProcessRepo {
//     procs: DashMap<Pid, Arc<Mutex<Process>>>,
// }

// impl ProcessRepo {
//     /// 创建一个新的 ProcessRepo 实例
//     pub fn new() -> Self {
//         ProcessRepo {
//             procs: DashMap::new(),
//         }
//     }

//     /// 清空所有进程
//     pub fn clear(&self) {
//         self.procs.clear();
//     }

//     /// 添加一个新的进程，如果 pid 已存在则返回错误
//     pub fn add_process(&self, pid: &Pid, proc: Process) -> Result<(), String> {
//         // 尝试插入，如果已存在则返回错误
//         let entry = self.procs.entry(*pid).or_try_insert_with(|| {
//             Arc::new(Mutex::new(proc))
//         });
//         match entry {
//             Ok(_) => Ok(()),
//             Err(_) => Err(format!("Process with pid {} already exists", pid)),
//         }
//     }

//     /// 获取指定 pid 的进程
//     pub fn get_process(&self, pid: &Pid) -> Option<Arc<Mutex<Process>>> {
//         self.procs.get(pid).map(|entry| Arc::clone(entry.value()))
//     }

//     /// 对每个进程执行函数 `f`
//     pub fn for_each<F>(&self, mut f: F)
//     where
//         F: FnMut(Pid, &Process),
//     {
//         self.procs.iter().for_each(|entry| {
//             let pid = *entry.key();
//             if let Ok(process) = entry.value().lock() {
//                 f(pid, &*process);
//             }
//         });
//     }

//     /// 对每个进程可变地执行函数 `f`
//     pub fn for_each_mut<F>(&self, mut f: F)
//     where
//         F: FnMut(Pid, &mut Process),
//     {
//         self.procs.iter().for_each(|entry| {
//             let pid = *entry.key();
//             if let Ok(mut process) = entry.value().lock() {
//                 f(pid, &mut *process);
//             }
//         });
//     }

//     /// 并行地将每个进程映射为另一个类型的结果
//     pub fn map_concurrent<F, R>(&self, f: F) -> Vec<R>
//     where
//         F: Fn(Pid, &Process) -> R + Send + Sync,
//         R: Send,
//     {
//         self.procs.par_iter()
//             .filter_map(|entry| {
//                 let pid = *entry.key();
//                 match entry.value().lock() {
//                     Ok(process) => Some(f(pid, &*process)),
//                     Err(_) => None,
//                 }
//             })
//             .collect()
//     }
// }