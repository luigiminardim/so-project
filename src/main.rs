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
use parsers::files_parser::OperationType;

mod resources;

// cargo run -- input/processes.txt input/files.txt

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("\n\nParsing Input \n");
    let processes_path = "input/processes.txt";
    let files_path = "input/files.txt";

    // Parse processes
    let mut processes_table = parsers::processes_parser::parse(processes_path);

    // Parse files
    let (num_blocks, alloc_disk_blocks, sysfile_operations) =
        parsers::files_parser::parse(files_path);

    // Simulations
    println!("\n\nSimulating Dispatcher \n");
    let mut memory_manager = memory::MemoryManager::new();
    let mut resource_manager = resources::ResourceManager::new();

    println!("\n\nSimulating File System \n");
    let mut file_manager = files::FileManager::new(num_blocks, alloc_disk_blocks);

    let mut index = 1;
    for operation in sysfile_operations {
        match operation {
            OperationType::Create {
                process_id,
                file_name,
                file_size,
            } => {
                let alloc_segment = file_manager.create_file(
                    &mut processes_table[process_id],
                    file_name,
                    num_blocks,
                );
                println!("alloc_segment = {:?}", alloc_segment);
                if alloc_segment.is_some() {
                    println!("Operação {index} => Sucesso");
                    println!("  O processo {process_id} criou o arquivo {file_name} com {file_size} blocos");
                } else {
                    println!("Operação {index} => Falha");
                    println!("  O processo {process_id} não pôde criar o arquivo {file_name} com {file_size} blocos");
                }
            }
            OperationType::Erase {
                process_id,
                file_name,
            } => {
                let result = file_manager.delete_file(&mut processes_table[process_id], file_name);
                if result.is_ok() {
                    println!("Operação {index} => Sucesso");
                    println!("  O processo {process_id} deletou o arquivo {file_name}");
                } else {
                    println!("Operação {index} => Falha");
                    println!("  O processo {process_id} não pôde deletar o arquivo {file_name}");
                }
            }
        }
        index += 1;
        println!("\n");
    }
}
