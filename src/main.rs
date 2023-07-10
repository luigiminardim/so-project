mod memory;
use memory::MemoryManager;

fn main() {
    let mut memory_manager = MemoryManager::new();
    let first_partition = memory_manager.allocate(8).unwrap();
    memory_manager.free(first_partition);
}
