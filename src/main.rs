use std::fs::File;
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
use crate::files::FileManager;
use crate::process::{DiskOperation, Interruption};
use crate::queues::ProcessManager;
use crate::resources::ResourceManager;

mod resources;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("\n\nParsing Input \n");
    let processes_path = "input/processes.txt";
    let files_path = "input/files.txt";
    // Parse processes
    let mut processes_definitions = parsers::processes_parser::parse(processes_path);
    // Parse files
    let (num_blocks, alloc_disk_blocks, disk_operation_definitions) =
        parsers::files_parser::parse(files_path);

    let mut dispatcher = Dispatcher::new(processes_definitions, disk_operation_definitions);
    let mut process_manager = ProcessManager::new();
    let mut resource_manager = ResourceManager::new();
    let mut file_manager = FileManager::new(num_blocks, alloc_disk_blocks);

    println!("\n\nStarting simulation\n");

    let mut timestamp = 0;
    while true {
        let new_processes = dispatcher.generate_new_processes(timestamp);
        for process in new_processes {
            process_manager.add_process(process, timestamp);
        }
        process_manager.on_tick(timestamp);
        let mut current_process = process_manager.get_current_process();
        if let Some(process) = current_process {
            let process_tick_result = process.on_tick();
            match process_tick_result {
                Interruption::None => {}
                Interruption::Terminate => {
                    process_manager.terminate_current_process(timestamp);
                }
                Interruption::AllocResource { resource } => {
                    // I don't know how to implement AllocResource, because I don't know when to release the resource.
                    // I also tried to add the process to the process_manager again, but it didn't compile >:(

                    // let blocked_process = process_manager.block_current_process(timestamp);
                    // if let Some(blocked_process) = blocked_process {
                    //     resource_manager.request(blocked_process, resource);
                    // }
                    // process_manager.add_process(process, timestamp);
                }
                Interruption::DiskInstruction { instruction } => match instruction {
                    DiskOperation::Create {
                        file_name,
                        num_blocks,
                    } => {
                        if let Some(segment) =
                            file_manager.create_file(process, file_name, num_blocks)
                        {
                            println!("File {file_name} was created: {:?}", segment);
                        } else {
                            println!("File {file_name} wasn't created");
                        }
                    }
                    DiskOperation::Delete { file_name } => {
                        let result = file_manager.delete_file(process, file_name);
                        if result.is_ok() {
                            println!("File {file_name} was deleted");
                        } else {
                            println!("File {file_name} wasn't deleted");
                        }
                    }
                },
            }
        }
        timestamp += 1;
    }
}
