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
use crate::memory::MemoryManager;
use crate::process::{DiskOperation, Interruption};
use crate::queues::ProcessManager;
use crate::resources::ResourceManager;

mod resources;

fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    let (default_processes_path, default_files_path) = (
        String::from("input/processes.txt"),
        String::from("input/files.txt"),
    );
    let processes_path = argv.get(1).unwrap_or(&default_processes_path);
    let files_path = argv.get(2).unwrap_or(&default_files_path);
    println!("Processes path: {}", processes_path);
    println!("Files path: {}", files_path);
    // Parse processes
    let processes_definitions = parsers::processes_parser::parse(processes_path);
    // Parse files
    let (num_blocks, alloc_disk_blocks, disk_operation_definitions) =
        parsers::files_parser::parse(files_path);

    let mut memory_manager = MemoryManager::new();
    let mut file_manager = FileManager::new(num_blocks, alloc_disk_blocks);
    let mut dispatcher = Dispatcher::new(processes_definitions, disk_operation_definitions);
    let mut process_manager = ProcessManager::new();
    let mut resource_manager = ResourceManager::new();
    let mut timestamp = 0;
    while dispatcher.has_more_processes(timestamp) || process_manager.has_more_processes() {
        let new_processes = dispatcher.generate_new_processes(&mut memory_manager, timestamp);
        for process in new_processes {
            process_manager.add_process(process, timestamp);
        }
        if let Some(current_process) = process_manager.get_current_process() {
            match current_process.on_tick() {
                Interruption::None => {
                    println!(
                        "Process {} CPU instruction\n",
                        current_process.software_context.id,
                    )
                }
                Interruption::Terminate => {
                    let teminated_process = process_manager.terminate_current_process();
                    if let Some(mut terminated_process) = teminated_process {
                        let unblocked_processes =
                            resource_manager.release_resources(&mut terminated_process);
                        for unblocked_process in unblocked_processes {
                            process_manager.add_process(unblocked_process, timestamp);
                        }
                        memory_manager.free(terminated_process.address_space);
                    }
                }
                Interruption::AllocResource { resource } => {
                    if let Some(blocked_process) = process_manager.block_current_process() {
                        if let Some(unblocked_process) =
                            resource_manager.request(blocked_process, resource)
                        {
                            process_manager.add_process(unblocked_process, timestamp);
                        }
                    }
                }
                Interruption::DiskInterruption { instruction } => {
                    if let Some(mut blocked_process) = process_manager.block_current_process() {
                        match instruction {
                            DiskOperation::Create {
                                file_name,
                                num_blocks,
                            } => {
                                file_manager.create_file(
                                    &mut blocked_process,
                                    file_name,
                                    num_blocks,
                                );
                                process_manager.add_process(blocked_process, timestamp);
                            }
                            DiskOperation::Delete { file_name } => {
                                let _ = file_manager.delete_file(&mut blocked_process, file_name);
                                process_manager.add_process(blocked_process, timestamp);
                            }
                        }
                    }
                }
            }
        }
        timestamp += 1;
        process_manager.on_tick(timestamp);
    }
}
