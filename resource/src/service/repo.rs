
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "repo_local")] {
        pub mod local_process_repo;
        pub use local_process_repo::ProcessRepo;
    }
    else if #[cfg(feature = "repo_hashmap")] {
        pub mod hashmap_process_repo;
        pub use hashmap_process_repo::ProcessRepo;
    } else {
        pub mod dashmap_process_repo;
        pub use dashmap_process_repo::ProcessRepo;
    }
}