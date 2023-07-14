use crate::{
    memory::MemoryManager,
    parsers::{files_parser::DiskOperationDefinition, processes_parser::ProcessDefinition},
    process::{DiskOperation, Process},
};

pub struct Dispatcher {
    processes_definitions: Vec<ProcessDefinition>,
    disk_operation_definitions: Vec<DiskOperationDefinition>,
}

impl Dispatcher {
    pub fn new(
        processes_definitions: Vec<ProcessDefinition>,
        disk_operation_definitions: Vec<DiskOperationDefinition>,
    ) -> Self {
        Self {
            processes_definitions,
            disk_operation_definitions,
        }
    }

    pub fn generate_new_processes(
        &mut self,
        mut memory_manager: &mut MemoryManager,
        timestamp: usize,
    ) -> Vec<Process> {
        let mut new_processes = Vec::new();
        for process_definition in self.processes_definitions.iter() {
            if process_definition.init_time == timestamp {
                if let Some(process) = self.build_process(process_definition, &mut memory_manager) {
                    process.println();
                    new_processes.push(process);
                } else {
                    println!(
                        "Process {} could not be created due to lack of memory\n",
                        process_definition.id
                    );
                }
            }
        }
        new_processes
    }

    fn build_process(
        &self,
        process_definition: &ProcessDefinition,
        memory_manager: &mut MemoryManager,
    ) -> Option<Process> {
        let process_disk_ops = self
            .disk_operation_definitions
            .iter()
            .filter_map(
                |disk_operation_definition| match disk_operation_definition {
                    DiskOperationDefinition::Create {
                        process_id,
                        file_name,
                        file_size,
                    } if *process_id == process_definition.id => Some(DiskOperation::Create {
                        file_name: *file_name,
                        num_blocks: *file_size,
                    }),
                    DiskOperationDefinition::Erase {
                        process_id,
                        file_name,
                    } if *process_id == process_definition.id => Some(DiskOperation::Delete {
                        file_name: *file_name,
                    }),
                    _ => None,
                },
            )
            .collect();
        let address_space = memory_manager.alloc(
            process_definition.priority,
            process_definition.num_memory_blocks,
        )?;
        let new_process = Process::new(
            process_definition.id,
            process_definition.priority,
            process_definition.cpu_time,
            process_definition.use_printer,
            process_definition.use_scanner,
            process_definition.use_modem,
            process_definition.use_sata,
            process_disk_ops,
            address_space,
        );
        Some(new_process)
    }

    pub fn has_more_processes(&self, timestamp: usize) -> bool {
        self.processes_definitions
            .iter()
            .any(|process_definition| process_definition.init_time >= timestamp)
    }
}
