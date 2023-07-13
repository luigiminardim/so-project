use crate::structures::segment_list::Segment;

#[derive(Debug)]
pub enum OperationType {
    Create {
        process_id: usize,
        file_name: char,
        file_size: usize,
    },
    Erase {
        process_id: usize,
        file_name: char,
    },
}

pub fn parse(files_path: &str) -> (usize, Vec<(char, Segment)>, Vec<OperationType>) {
    let file_string = std::fs::read_to_string(files_path).unwrap();
    let mut lines = file_string.lines();

    let num_blocks = lines.next().unwrap().parse::<usize>().unwrap();
    let num_disk_segments = lines.next().unwrap().parse::<usize>().unwrap();
    println!("num_blocks = {num_blocks}");
    println!("num_segments = {num_disk_segments}");

    let mut alloc_disk_blocks: Vec<(char, Segment)> = Vec::new();
    for _ in 0..num_disk_segments {
        let params: Vec<&str> = lines.next().unwrap().split(", ").collect();
        let file_name = params[0].chars().next().unwrap();
        let offset = params[1].parse::<usize>().unwrap();
        let length = params[2].parse::<usize>().unwrap();
        alloc_disk_blocks.push((file_name, Segment { offset, length }));
        println!("(file = {file_name}, offset = {offset}, length = {length})");
    }

    let mut sysfile_operations: Vec<OperationType> = Vec::new();
    while let Some(line) = lines.next() {
        let params: Vec<&str> = line.split(", ").collect();
        let process_id = params[0].parse::<usize>().unwrap();
        let operation_code = params[1].parse::<usize>().unwrap();
        let file_name = params[2].chars().next().unwrap();
        if operation_code == 0 {
            let file_size = params[3].parse::<usize>().unwrap();
            println!("processso {process_id} cria arquivo {file_name} com {file_size} blocos");
            sysfile_operations.push(OperationType::Create {
                process_id,
                file_name,
                file_size,
            });
        } else {
            println!("processso {process_id} deleta arquivo {file_name}");
            sysfile_operations.push(OperationType::Erase {
                process_id,
                file_name,
            });
        }
    }

    (num_blocks, alloc_disk_blocks, sysfile_operations)
}
