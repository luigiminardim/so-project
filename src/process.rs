use crate::resources::Resource;

pub struct SoftwareContext {
    pub priority: usize,
    pub cpu_time: u32,
    pub printer_request_code: u32,
    pub scanner_request: bool,
    pub modem_request: bool,
    pub disk_request_code: u32,

    pub files_created: Vec<char>,
    pub resources: Vec<Resource>,
}

struct HardwareContext {
    pc: u32,
    initialization_time: u32,
    memory_blocks: u32,
}

pub struct Process {
    hardware_context: HardwareContext,
    pub software_context: SoftwareContext,
}

impl Process {
    pub fn new(
        initialization_time: u32,
        priority: usize,
        cpu_time: u32,
        memory_blocks: u32,
        printer_request_code: u32,
        scanner_request: bool,
        modem_request: bool,
        disk_request_code: u32,
    ) -> Process {
        Process {
            hardware_context: HardwareContext {
                pc: 0,
                initialization_time,
                memory_blocks,
            },
            software_context: SoftwareContext {
                priority,
                cpu_time,
                printer_request_code,
                scanner_request,
                modem_request,
                disk_request_code,
                files_created: Vec::new(),
                resources: Vec::new(),
            },
        }
    }
}
