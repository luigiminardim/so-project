use crate::structures::segment_list::{Segment, SegmentList};

pub struct MemoryManager {
    real_time_partition: SegmentList,
    user_partition: SegmentList,
}

const REAL_TIME_PARTITION_SIZE: usize = 64;
const USER_PARTITION_SIZE: usize = 960;

#[derive(Debug, PartialEq)]
pub enum AllocMemoryError {
    Unavailable,
    Unsupported,
}

impl MemoryManager {
    pub fn new() -> Self {
        let real_time_initial_segment = Segment {
            offset: 0,
            length: REAL_TIME_PARTITION_SIZE,
        };
        let user_initial_segment = Segment {
            offset: REAL_TIME_PARTITION_SIZE,
            length: USER_PARTITION_SIZE,
        };
        Self {
            real_time_partition: SegmentList::new(vec![real_time_initial_segment]),
            user_partition: SegmentList::new(vec![user_initial_segment]),
        }
    }

    pub fn alloc(
        &mut self,
        process_priority: usize,
        size: usize,
    ) -> Result<Segment, AllocMemoryError> {
        match process_priority {
            0 => match self.real_time_partition.alloc(size) {
                None if size > REAL_TIME_PARTITION_SIZE => Err(AllocMemoryError::Unsupported),
                None => Err(AllocMemoryError::Unavailable),
                Some(segment) => Ok(segment),
            },
            _ => match self.user_partition.alloc(size) {
                None if size > USER_PARTITION_SIZE => Err(AllocMemoryError::Unsupported),
                None => Err(AllocMemoryError::Unavailable),
                Some(segment) => Ok(segment),
            },
        }
    }

    pub fn free(&mut self, segment: Segment) {
        if segment.offset < REAL_TIME_PARTITION_SIZE {
            self.real_time_partition.free(segment);
        } else {
            self.user_partition.free(segment);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod alloc {
        use super::*;

        #[test]
        fn real_time_process_allocs_from_first_partition() {
            let mut memory_manager = MemoryManager::new();
            let alloc_segment = memory_manager.alloc(0, 10);
            assert_eq!(
                alloc_segment,
                Ok(Segment {
                    offset: 0,
                    length: 10
                })
            );
        }

        #[test]
        fn user_process_allocs_from_second_partition() {
            let mut memory_manager = MemoryManager::new();
            let alloc_segment = memory_manager.alloc(1, 10);
            assert_eq!(
                alloc_segment,
                Ok(Segment {
                    offset: 64,
                    length: 10
                })
            );
        }

        #[test]
        fn real_time_process_dont_access_user_partition() {
            let mut memory_manager = MemoryManager::new();
            let alloc_segment = memory_manager.alloc(0, 65);
            assert_eq!(alloc_segment, Err(AllocMemoryError::Unsupported));
        }
    }

    mod free {
        use super::*;

        #[test]
        fn real_time_partition_segments_can_be_reused() {
            let mut memory_manager = MemoryManager::new();
            let alloc_segment = memory_manager.alloc(0, 10);
            assert_eq!(
                alloc_segment,
                Ok(Segment {
                    offset: 0,
                    length: 10
                })
            );
            memory_manager.free(alloc_segment.unwrap());
            let alloc_segment = memory_manager.alloc(0, 10);
            assert_eq!(
                alloc_segment,
                Ok(Segment {
                    offset: 0,
                    length: 10
                })
            );
        }

        #[test]
        fn user_partition_segments_can_be_reused() {
            let mut memory_manager = MemoryManager::new();
            let alloc_segment = memory_manager.alloc(1, 10);
            assert_eq!(
                alloc_segment,
                Ok(Segment {
                    offset: 64,
                    length: 10
                })
            );
            memory_manager.free(alloc_segment.unwrap());
            let alloc_segment = memory_manager.alloc(1, 10);
            assert_eq!(
                alloc_segment,
                Ok(Segment {
                    offset: 64,
                    length: 10
                })
            );
        }
    }
}
