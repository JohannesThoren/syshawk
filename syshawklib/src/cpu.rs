
use sysinfo::{CpuExt, SystemExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cpu {
    pub name: String,
    pub cores: usize,
    pub physical_cores: usize,
    pub usage: f32,
    pub per_core_usage: Vec<f32>,
    pub speed: f32,

}

impl Cpu {
    pub fn collect(sys: &mut sysinfo::System) -> Cpu {
        sys.refresh_all();

        let cpus = sys.cpus();
        let name = cpus[0].brand().to_string();
        let cores = cpus.len(); // Total cores (logical)
        let physical_cores = sys.physical_core_count().unwrap_or(cores);
        let usage = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cores as f32;
        let per_core_usage: Vec<f32> = cpus.iter().map(|cpu| cpu.cpu_usage()).collect();
        let speed = cpus[0].frequency() as f32; // Frequency in MHz

        Cpu {
            name,
            cores,
            physical_cores,
            usage,
            per_core_usage,
            speed,
        }
    }
}