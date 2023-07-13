use std::string;

mod files;
mod memory;
mod process;
mod queues;
mod structures {
    pub mod segment_list;
}
mod resources;

// cargo run -- input/processes.txt input/files.txt

fn main() {
    let memory_manager = memory::MemoryManager::new();

    let args: Vec<String> = std::env::args().collect();

    // Parse processes
    let processes_path = &args[1];
    for line in std::fs::read_to_string(processes_path).unwrap().lines() {
        let params: Vec<u32> = line
            .split(", ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        println!("process = {:?}", params);
        let new_process = process::Process::new(
            params[0],
            params[1] as usize,
            params[2],
            params[3],
            params[4],
            params[5] != 0,
            params[6] != 0,
            params[7],
        );
        // include new_process in a list of processes
    }

    // Parse files
    let files_path = &args[2];
    let file_string = std::fs::read_to_string(files_path).unwrap();
    let mut lines = file_string.lines();

    let number_disk_blocks = lines.next().unwrap().parse::<u32>().unwrap();
    let number_disk_segments = lines.next().unwrap().parse::<u32>().unwrap();
    println!("number_disk_blocks = {number_disk_blocks}");
    println!("number_disk_segments = {number_disk_segments}");

    for _ in 0..number_disk_segments {
        let params: Vec<&str> = lines.next().unwrap().split(", ").collect();
        let file_name = params[0].chars().next().unwrap();
        let offset = params[1].parse::<u32>().unwrap();
        let length = params[2].parse::<u32>().unwrap();
        println!("(file_name, offset, lenght) = ({file_name}, {offset}, {length})");
    }

    while let Some(line) = lines.next() {
        let params: Vec<&str> = line.split(", ").collect();
        let process_id = params[0].parse::<u32>().unwrap();
        let operation_code = params[1].parse::<u32>().unwrap();
        let file_name = params[2].chars().next().unwrap();
        if operation_code == 0 {
            let number_blocks = params[3].parse::<u32>().unwrap();
            println!("processso {process_id} cria arquivo {file_name} com {number_blocks} blocos");
        } else {
            println!("processso {process_id} deleta arquivo {file_name}");
        }
    }

    // Simulate processes
}
