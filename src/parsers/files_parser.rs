use crate::structures::segment_list::Segment;

pub fn parse(files_path: &str) -> (usize, Vec<(char, Segment)>) {
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

    while let Some(line) = lines.next() {
        let params: Vec<&str> = line.split(", ").collect();
        let process_id = params[0].parse::<usize>().unwrap();
        let operation_code = params[1].parse::<usize>().unwrap();
        let file_name = params[2].chars().next().unwrap();
        if operation_code == 0 {
            let number_blocks = params[3].parse::<usize>().unwrap();
            println!("processso {process_id} cria arquivo {file_name} com {number_blocks} blocos");
        } else {
            println!("processso {process_id} deleta arquivo {file_name}");
        }
    }

    (num_blocks, alloc_disk_blocks)
}
