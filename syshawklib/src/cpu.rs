use std::{thread, time::Duration};

use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, SystemExt};

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
    pub fn collect(sys: &mut sysinfo::System) -> Option<Cpu> {
        // Initial refresh to initialize the CPU usage data.
        sys.refresh_cpu();

        // Sleep for a short duration to allow the system to gather usage data.
        thread::sleep(Duration::from_millis(100));

        // Refresh again to get accurate CPU usage data.
        sys.refresh_cpu();

        // Get the list of CPUs.
        let cpus = sys.cpus();

        // If there are no CPUs, return None.
        if cpus.is_empty() {
            return None;
        }

        // Get the brand name of the CPU. Assuming all CPUs have the same brand.
        let name = cpus[0].brand().to_string();

        // Total cores (logical).
        let cores = cpus.len();

        // Physical cores, if available. Fall back to the number of logical cores if not available.
        let physical_cores = sys.physical_core_count().unwrap_or(cores);

        // Calculate the overall CPU usage by averaging the usage across all cores.
        let usage = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cores as f32;

        // Collect the CPU usage per core.
        let per_core_usage: Vec<f32> = cpus.iter().map(|cpu| cpu.cpu_usage()).collect();

        // Get the frequency of the first CPU (in MHz).
        let speed = cpus[0].frequency() as f32;

        // Return the CPU struct with the collected information.
        Some(Cpu {
            name,
            cores,
            physical_cores,
            usage,
            per_core_usage,
            speed,
        })
    }
}
