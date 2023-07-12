mod process;
mod memory;
mod files;
mod queues;

use memory::MemoryManager;
use process::Process;
use files::FileManager;

fn main() {
    let mut memory_manager = MemoryManager::new();
    let first_partition = memory_manager.allocate(8).unwrap();
    memory_manager.free(first_partition);

    let mut real_time_process = Process::new(0);

    let mut file_manager = FileManager::new(6, vec![]);
    file_manager.create_file(&mut real_time_process, 'A', 6);
    file_manager.delete_file(&real_time_process, 'A').unwrap();
}
