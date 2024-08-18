use std::{thread, time::Duration};

use serde::{Deserialize, Serialize};
use sysinfo;
use sysinfo::{CpuExt, System, SystemExt};

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
        sys.refresh_cpu();

        thread::sleep(sysinfo::System::MINIMUM_CPU_UPDATE_INTERVAL);

        sys.refresh_cpu();

        let cpus = sys.cpus();

        if cpus.is_empty() {
            return None;
        }

        let name = cpus[0].brand().to_string();

        let cores = cpus.len();

        let physical_cores = sys.physical_core_count().unwrap_or(cores);

        let usage = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cores as f32;

        let per_core_usage: Vec<f32> = cpus.iter().map(|cpu| cpu.cpu_usage()).collect();

        let total_frequency: u64 = cpus.iter().map(|processor| processor.frequency()).sum();

        // Calculate the number of processors
        let num_processors = cpus.len() as u64;

        // Compute the average frequency
        let average_frequency = if num_processors > 0 {
            total_frequency / num_processors
        } else {
            0
        };

        // Convert average frequency from Hz to MHz
        let average_frequency_mhz = average_frequency / 1_000_000;

        Some(Cpu {
            name,
            cores,
            physical_cores,
            usage,
            per_core_usage,
            speed: average_frequency as f32,
        })
    }
}
