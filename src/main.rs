use std::{string, vec};

mod files;
mod memory;
mod process;
mod queues;
mod structures {
    pub mod segment_list;
}
mod parsers {
    pub mod files_parser;
    pub mod processes_parser;
}
mod dispatcher;

use crate::dispatcher::Dispatcher;
use crate::process::Interruption;
use crate::queues::ProcessManager;
use crate::resources::ResourceManager;

mod resources;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("\n\nParsing Input \n");
    let processes_path = "input/processes.txt";
    let files_path = "input/files2.txt";
    // Parse processes
    let mut processes_definitions = parsers::processes_parser::parse(processes_path);
    // Parse files
    let (num_blocks, alloc_disk_blocks, disk_operation_definitions) =
        parsers::files_parser::parse(files_path);

    let mut dispatcher = Dispatcher::new(processes_definitions, disk_operation_definitions);
    let mut process_manager = ProcessManager::new();
    let mut resource_manager = ResourceManager::new();

    let mut timestamp = 0;
    while true {
        let new_processes = dispatcher.generate_new_processes(timestamp);
        for process in new_processes {
            process_manager.add_process(process, timestamp);
        }
        let mut current_process = process_manager.get_current_process();
        if let Some(current_process) = current_process
        match current_process {
            None => {}
            Some(process) => {
                let process_tick_result = process.on_tick();
                match process_tick_result {
                    Interruption::None => {}
                    Interruption::Terminate => {
                        process_manager.terminate_current_process(timestamp);
                    }
                    Interruption::AllocResource { resource } => {
                        let blocked_process = process_manager.block_current_process(timestamp);
                        match  {
                            None => {}
                            Some(process) => {
                                resource_manager.request(process, resource);
                            }
                        }
                        resource_manager.request(blocked_process, resource)
                    }
                }
            }
        }
        timestamp += 1;
    }
}
