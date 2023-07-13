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

mod resources;

// cargo run -- input/processes.txt input/files.txt

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("\n\n Parsing Input \n");
    // Parse processes
    let processes_path = &args[1];
    let processes_table = parsers::processes_parser::parse(processes_path);
    // println!("processes_table = {:?}", processes_table);

    // Parse files
    let files_path = &args[2];
    let (num_blocks, alloc_disk_blocks) = parsers::files_parser::parse(files_path);

    // Simulate process
    println!("\n\n Simulating Dispatcher \n");
    let memory_manager = memory::MemoryManager::new();
    let resource_manager = resources::ResourceManager::new();

    println!("\n\n Simulating File System \n");
    let file_manager = files::FileManager::new(num_blocks, alloc_disk_blocks);
}
