export type System = {
    hostname: string;
    uptime: number;
    cpu: Cpu;
    memory: Memory;
    fs: Fs[];
}

export type Cpu = {
    name: string;
    cores: number;
    physical_cores: number;
    usage: number;
    per_core_usage: number[];
    speed: number;
}

export type Memory = {
    total_memory: number;
    used_memory: number;
    available_memory: number;
    swap_used: number;
    swap_total: number;
}

export type Fs = {
    drive_name: string;
    mount_point: string;
    fs_type: string;
    size: number;
    used: number;
    is_removable: boolean;
}