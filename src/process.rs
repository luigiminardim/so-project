use crate::{resources::Resource, structures::segment_list::Segment};

#[derive(Debug, Clone)]
pub enum DiskOperation {
    Create { file_name: char, num_blocks: usize },
    Delete { file_name: char },
}

#[derive(Debug, Clone)]
pub enum Interruption {
    None,
    AllocResource { resource: Resource },
    DiskInterruption { instruction: DiskOperation },
    Terminate,
}

#[derive(Debug)]
pub struct SoftwareContext {
    pub id: usize,
    pub priority: usize,
    pub files_created: Vec<char>,
    pub resources: Vec<Resource>,
    cpu_time: usize,
    instructions: Vec<Interruption>,
}

#[derive(Debug)]
struct HardwareContext {
    pc: usize,
}

#[derive(Debug)]
pub struct Process {
    hardware_context: HardwareContext,
    pub software_context: SoftwareContext,
    pub address_space: Segment,
}

impl Process {
    pub fn new(
        id: usize,
        priority: usize,
        cpu_time: usize,
        use_printer: bool,
        use_scanner: bool,
        use_modem: bool,
        use_sata: bool,
        disk_operations: Vec<DiskOperation>,
        address_space: Segment,
    ) -> Process {
        let instructions = Process::build_instructions(
            use_printer,
            use_scanner,
            use_modem,
            use_sata,
            disk_operations,
        );
        Process {
            hardware_context: HardwareContext { pc: 0 },
            software_context: SoftwareContext {
                id,
                priority,
                cpu_time,
                instructions,
                files_created: Vec::new(),
                resources: Vec::new(),
            },
            address_space,
        }
    }

    fn build_instructions(
        use_printer: bool,
        use_scanner: bool,
        use_modem: bool,
        use_sata: bool,
        disk_operations: Vec<DiskOperation>,
    ) -> Vec<Interruption> {
        let mut instructions = Vec::new();
        if use_scanner {
            instructions.push(Interruption::AllocResource {
                resource: Resource::Scanner,
            });
        }
        if use_printer {
            instructions.push(Interruption::AllocResource {
                resource: Resource::Printer,
            });
        }
        if use_modem {
            instructions.push(Interruption::AllocResource {
                resource: Resource::Modem,
            });
        }
        if use_sata {
            instructions.push(Interruption::AllocResource {
                resource: Resource::SataDevice,
            });
        }
        for disk_operation in disk_operations {
            instructions.push(Interruption::DiskInterruption {
                instruction: disk_operation,
            });
        }
        instructions
    }

    pub fn on_tick(&mut self) -> Interruption {
        if self.hardware_context.pc
            >= self.software_context.cpu_time + self.software_context.instructions.len()
        {
            return Interruption::Terminate;
        }
        let interruption = self
            .software_context
            .instructions
            .get(self.hardware_context.pc)
            .unwrap_or(&Interruption::None);
        self.hardware_context.pc += 1;
        interruption.clone()
    }

    pub fn println(&self) {
        println!(
            "Process {{ pid: {}, offset: {}, blocks: {}, priority: {}, time: {}, instructions: {:?} }}\n",
            self.software_context.id,
            self.address_space.offset,
            self.address_space.length,
            self.software_context.priority,
            self.software_context.cpu_time,
            self.software_context.instructions.iter().filter_map(|instruction| match instruction {
                Interruption::AllocResource { resource } => Some(resource),
                _ => None,
            }).collect::<Vec<_>>(),
        )
    }
}
