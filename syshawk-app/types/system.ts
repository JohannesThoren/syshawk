export type System = {
    hostname: string;
    uptime: number;
    cpu: Cpu;
    memory: Memory;
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