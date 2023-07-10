struct SoftwareContext {}

struct HardwareContext {
    pc: u32,
}

struct AddressSpace {
    offset: u32,
    
}

pub struct Process {
    hardware_context: HardwareContext,
    software_context: SoftwareContext,
}

impl Process {

}
