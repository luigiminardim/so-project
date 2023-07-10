#[derive(Debug, Clone, Copy)]
pub struct MemoryPartition {
    pub offset: u32,
    pub num_blocks: u32,
}

pub struct MemoryManager {
    free_partitions: Vec<MemoryPartition>,
}

impl MemoryManager {
    pub fn new() -> Self {
        let mut free_partitions = Vec::new();
        free_partitions.push(MemoryPartition {
            offset: 0,
            num_blocks: 1024,
        });
        Self { free_partitions }
    }

    pub fn allocate(&mut self, num_blocks: u32) -> Option<MemoryPartition> {
        for it in self.free_partitions.iter_mut() {
            if it.num_blocks >= num_blocks {
                let partition = MemoryPartition {
                    offset: it.offset,
                    num_blocks,
                };
                it.offset += num_blocks;
                it.num_blocks -= num_blocks;
                return Some(partition);
            }
        }
        None
    }

    pub fn free(&mut self, memory_partition: MemoryPartition) {
        let mut index = 0;
        for it in self.free_partitions.iter() {
            if it.offset > memory_partition.offset {
                break;
            }
            index += 1;
        }
        self.free_partitions.insert(index, memory_partition);
        self.merge_free_partitions();
    }

    fn merge_free_partitions(&mut self) {
        let mut index = 0;
        while index < self.free_partitions.len() - 2 {
            let current = self.free_partitions[index];
            let next = self.free_partitions[index + 1];
            if current.offset + current.num_blocks == next.offset {
                self.free_partitions[index].num_blocks += next.num_blocks;
                self.free_partitions.remove(index + 1);
            } else {
                index += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_alloc() {
        let mut memory_manager = MemoryManager::new();
        let first_alloc = memory_manager.allocate(8).unwrap();
        let second_alloc = memory_manager.allocate(16).unwrap();
        assert_eq!(first_alloc.offset, 0);
        assert_eq!(first_alloc.num_blocks, 8);
        assert_eq!(second_alloc.offset, 8);
        assert_eq!(second_alloc.num_blocks, 16);
    }

    mod free {
        use super::*;

        #[test]
        fn test_free() {
            let mut memory_manager = MemoryManager::new();
            let first_partition = memory_manager.allocate(8).unwrap();
            let _second_partition = memory_manager.allocate(16).unwrap();
            memory_manager.free(first_partition);
            let second_alloc_of_first_partition = memory_manager.allocate(8).unwrap();
            assert_eq!(second_alloc_of_first_partition.offset, 0);
            assert_eq!(second_alloc_of_first_partition.num_blocks, 8);
        }

        #[test]
        fn test_merge_prev() {
            let mut memory_manager = MemoryManager::new();
            let first_partition = memory_manager.allocate(8).unwrap();
            let second_partition = memory_manager.allocate(16).unwrap();
            let _third_partition = memory_manager.allocate(8).unwrap();
            memory_manager.free(first_partition);
            memory_manager.free(second_partition);
            let alloc = memory_manager.allocate(24).unwrap();
            assert_eq!(alloc.offset, 0);
            assert_eq!(alloc.num_blocks, 24);
        }

        #[test]
        fn test_merge_next() {
            let mut memory_manager = MemoryManager::new();
            let first_partition = memory_manager.allocate(8).unwrap();
            memory_manager.free(first_partition);
            let alloc = memory_manager.allocate(8).unwrap();
            assert_eq!(alloc.offset, 0);
            assert_eq!(alloc.num_blocks, 8);
        }

        #[test]
        fn test_merge_prev_and_next() {
            let mut memory_manager = MemoryManager::new();
            let first_partition = memory_manager.allocate(8).unwrap();
            let second_partition = memory_manager.allocate(16).unwrap();
            memory_manager.free(first_partition);
            memory_manager.free(second_partition);
            let alloc = memory_manager.allocate(24).unwrap();
            assert_eq!(alloc.offset, 0);
            assert_eq!(alloc.num_blocks, 24);
        }
    }
}
