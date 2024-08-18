use serde::{Deserialize, Serialize};
use sysinfo::SystemExt;
use crate::{cpu, memory};
use crate::cpu::Cpu;
use crate::memory::Memory;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct System {
    pub hostname: String,
    pub uptime: u64,
    pub cpu: Cpu,
    pub memory: Memory,
}


impl System {
    pub fn collect() -> System {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        let hostname = sys.host_name().unwrap_or_else(|| "Unknown".to_string());
        let uptime = sys.uptime();

        let cpu = cpu::Cpu::collect(&mut sys);
        let memory = memory::Memory::collect(&mut sys);

        System {
            hostname,
            uptime,
            cpu,
            memory,
        }
    }
}