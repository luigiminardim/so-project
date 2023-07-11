pub struct SoftwareContext {
    pub priority: usize,
    pub files_created: Vec<char>,
}

struct HardwareContext {
    pc: u32,
}

struct AddressSpace {
    offset: u32,
    
}

pub struct Process {
    hardware_context: HardwareContext,
    pub software_context: SoftwareContext,
}

impl Process {
    pub fn new(priority: usize) -> Process {
        Process {
            hardware_context: HardwareContext { pc: 0 },
            software_context: SoftwareContext {
                priority,
                files_created: Vec::new(),
            },
        }
    }
}
