use anyhow::Result;
use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub total_memory: u64,
    pub used_memory: u64,
    pub available_memory: u64,
    pub swap_used: u64,
    pub swap_total: u64,
}

impl Memory {
    pub fn collect(sys: &mut sysinfo::System) -> Memory {
        sys.refresh_all();

        let total_memory = sys.total_memory();
        let available_memory = sys.available_memory();
        let used_memory = total_memory - available_memory;
        let swap_total = sys.total_swap();
        let swap_used = sys.used_swap();

        Memory {
            total_memory,
            used_memory,
            available_memory,
            swap_used,
            swap_total,
        }
    }
}