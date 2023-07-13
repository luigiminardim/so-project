use crate::process::Process;

pub fn parse(processes_path: &str) -> Vec<Process> {
    let mut processes_table: Vec<Process> = Vec::new();
    for line in std::fs::read_to_string(processes_path).unwrap().lines() {
        let params: Vec<u32> = line
            .split(", ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        println!("process = {:?}", params);
        let new_process = Process::new(
            params[0],
            params[1] as usize,
            params[2],
            params[3],
            params[4],
            params[5] != 0,
            params[6] != 0,
            params[7],
        );
        processes_table.push(new_process);
    }
    processes_table
}
