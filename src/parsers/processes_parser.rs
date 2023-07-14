pub struct ProcessDefinition {
    pub id: usize,
    pub init_time: usize,
    pub priority: usize,
    pub cpu_time: usize,
    pub num_memory_blocks: usize,
    pub use_printer: bool,
    pub use_scanner: bool,
    pub use_modem: bool,
    pub use_sata: bool,
}

pub fn parse(processes_path: &str) -> Vec<ProcessDefinition> {
    let mut process_definitions = Vec::new();
    for (id, line) in std::fs::read_to_string(processes_path)
        .unwrap()
        .lines()
        .enumerate()
    {
        let params: Vec<usize> = line
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        println!("process = {:?}", params);
        process_definitions.push(ProcessDefinition {
            id,
            init_time: params[0],
            priority: params[1],
            cpu_time: params[2],
            num_memory_blocks: params[3],
            use_printer: params[4] != 0,
            use_scanner: params[5] != 0,
            use_modem: params[6] != 0,
            use_sata: params[7] != 0,
        });
    }
    process_definitions
}
