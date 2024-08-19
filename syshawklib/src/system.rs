use serde::{Deserialize, Serialize};
use sysinfo::SystemExt;
use crate::fs::Fs;
use crate::{cpu, memory};
use crate::cpu::Cpu;
use crate::memory::Memory;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct System {
    pub hostname: String,
    pub uptime: u64,
    pub cpu: Option<Cpu>,
    pub memory: Option<Memory>,
    pub fs: Option<Vec<Fs>>
}


impl System {
    pub fn collect() -> System {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        let hostname = sys.host_name().unwrap_or_else(|| "Unknown".to_string());
        let uptime = sys.uptime();

        let cpu = Some(cpu::Cpu::collect(&mut sys).unwrap());
        let memory = Some(memory::Memory::collect(&mut sys));
        let fs = Some(Fs::collect(&sys));

        System {
            hostname,
            uptime,
            cpu,
            memory,
            fs
        }
    }
}