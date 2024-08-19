use serde::{Deserialize, Serialize};
use sysinfo::{DiskExt, System, SystemExt};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fs {
    pub drive_name: String,
    pub mount_point: String,
    pub fs_type: String,
    pub size: usize,
    pub used: usize,
    pub is_removable: bool,
}

impl Fs {
    pub fn collect(sys: &System) -> Vec<Self> {
        sys.disks()
            .iter()
            .map(|disk| {
                let drive_name = disk.name().to_str().unwrap_or("Unknown").to_string();
                let mount_point = disk.mount_point().to_str().unwrap_or("Unknown").to_string();
                let size = disk.total_space() as usize;
                let used = (disk.total_space() - disk.available_space()) as usize;
                let fs_type = String::from_utf8_lossy(disk.file_system()).into_owned();
                let is_removable = disk.is_removable();

                Fs {
                    drive_name,
                    mount_point,
                    fs_type,
                    size,
                    used,
                    is_removable,
                }
            })
            .collect()
    }
}
