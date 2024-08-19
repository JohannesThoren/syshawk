pub use sysinfo;

pub mod cpu;
pub mod memory;
pub mod system;
pub mod process;
pub mod fs;

#[cfg(test)]
mod tests {
    use sysinfo::SystemExt;
    use crate::{cpu, memory, system};



    #[test]
    fn test_cpu_information() {
        let mut sys = sysinfo::System::new_all();

        let cpu = cpu::Cpu::collect(&mut sys);
        println!("{:?}", cpu)
    }

    #[test]
    fn test_mem_information() {
        let mut sys = sysinfo::System::new_all();

        let memory = memory::Memory::collect(&mut sys);
        println!("{:?}", memory)
    }

    #[test]
    fn test_sys_info() {
        let mut sys = sysinfo::System::new_all();

        let sys = system::System::collect();
        println!("{:?}", sys)
    }
}
