use std::collections::HashMap;

use crate::process::Process;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DiskSegment {
    pub offset: usize,
    pub num_blocks: usize,
}

pub struct FileManager {
    free_segments: Vec<DiskSegment>,
    alloc_map: HashMap<char, DiskSegment>,
}

#[derive(Debug, PartialEq)]
pub enum DeleteFileError {
    NotFound,
    Unauthorized,
}

impl FileManager {
    pub fn new(num_blocks: usize, alloc_disk_blocks: Vec<(char, DiskSegment)>) -> FileManager {
        let free_segments = vec![DiskSegment {
            offset: 0,
            num_blocks,
        }];
        let alloc_map = alloc_disk_blocks.clone().into_iter().collect();
        let mut file_manager = FileManager {
            free_segments,
            alloc_map,
        };
        for (_, alloc_segment) in alloc_disk_blocks.iter() {
            file_manager.alloc_disk_segment(alloc_segment);
        }
        file_manager
    }

    fn alloc_disk_segment(&mut self, to_remove: &DiskSegment) -> Option<()> {
        let free_segments = &mut self.free_segments;
        let alloc_index = free_segments.iter().position(|s| {
            s.offset <= to_remove.offset
                && s.offset + s.num_blocks >= to_remove.offset + to_remove.num_blocks
        });
        match alloc_index {
            None => return None,
            Some(index) => {
                let left_remaining = DiskSegment {
                    offset: free_segments[index].offset,
                    num_blocks: to_remove.offset - free_segments[index].offset,
                };
                let right_remaining = DiskSegment {
                    offset: to_remove.offset + to_remove.num_blocks,
                    num_blocks: free_segments[index].offset + free_segments[index].num_blocks
                        - to_remove.offset
                        - to_remove.num_blocks,
                };
                if left_remaining.num_blocks == 0 && right_remaining.num_blocks == 0 {
                    free_segments.remove(index);
                } else if left_remaining.num_blocks == 0 {
                    free_segments[index] = right_remaining;
                } else if right_remaining.num_blocks == 0 {
                    free_segments[index] = left_remaining;
                } else {
                    free_segments[index] = left_remaining;
                    free_segments.insert(index + 1, right_remaining);
                }
                return Some(());
            }
        };
    }

    pub fn create_file(
        &mut self,
        process: &mut Process,
        file_name: char,
        num_blocks: usize,
    ) -> Option<DiskSegment> {
        let base_segment = self
            .free_segments
            .iter()
            .find(|s| s.num_blocks >= num_blocks)?;
        let alloc_segment = DiskSegment {
            offset: base_segment.offset,
            num_blocks,
        };
        self.alloc_disk_segment(&alloc_segment)?;
        self.alloc_map.insert(file_name, alloc_segment);
        process.software_context.files_created.push(file_name);
        Some(alloc_segment)
    }

    pub fn delete_file(
        &mut self,
        process: &Process,
        file_name: char,
    ) -> Result<(), DeleteFileError> {
        let is_authorized = process.software_context.priority == 0
            || process.software_context.files_created.contains(&file_name);
        if !is_authorized {
            return Err(DeleteFileError::Unauthorized);
        }
        let disk_segment = self
            .alloc_map
            .remove(&file_name)
            .ok_or(DeleteFileError::NotFound)?;
        self.free_disk_segment(disk_segment);
        Ok(())
    }

    fn free_disk_segment(&mut self, to_free: DiskSegment) {
        let index = self
            .free_segments
            .iter()
            .position(|segment| segment.offset > to_free.offset)
            .unwrap_or(self.free_segments.len());
        let has_to_merge_left = index > 0
            && self.free_segments[index - 1].offset + self.free_segments[index - 1].num_blocks
                >= to_free.offset;
        let has_to_merge_right = index < self.free_segments.len()
            && to_free.offset + to_free.num_blocks >= self.free_segments[index].offset;
        match (has_to_merge_left, has_to_merge_right) {
            (false, false) => {
                self.free_segments.insert(index, to_free);
            }
            (true, false) => {
                self.free_segments[index - 1].num_blocks =
                    to_free.offset + to_free.num_blocks - self.free_segments[index - 1].offset;
            }
            (false, true) => {
                self.free_segments[index].num_blocks = self.free_segments[index].offset
                    + self.free_segments[index].num_blocks
                    - to_free.offset;
                self.free_segments[index].offset = to_free.offset;
            }
            (true, true) => {
                self.free_segments[index - 1].num_blocks = self.free_segments[index].offset
                    + self.free_segments[index].num_blocks
                    - self.free_segments[index - 1].offset;
                self.free_segments.remove(index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Creates a FileManager with the following disk layout:
     * ['A', ' ', 'B', 'B', ' ', ' ']
     */
    fn create_test_file_manager() -> FileManager {
        let num_blocks: usize = 6;
        let alloc_disk_blocks: Vec<(char, DiskSegment)> = vec![
            (
                'A',
                DiskSegment {
                    offset: 0,
                    num_blocks: 1,
                },
            ),
            (
                'B',
                DiskSegment {
                    offset: 2,
                    num_blocks: 2,
                },
            ),
        ];
        FileManager::new(num_blocks, alloc_disk_blocks.clone())
    }

    fn create_process_mock(priority: usize) -> Process {
        Process::new(priority)
    }

    mod create_file {
        use super::*;

        #[test]
        fn test_create_file() {
            let mut file_manager = create_test_file_manager();
            let mut process = create_process_mock(0);
            assert_eq!(file_manager.create_file(&mut process, 'C', 3), None);
            assert_eq!(
                file_manager.create_file(&mut process, 'C', 2),
                Some(DiskSegment {
                    offset: 4,
                    num_blocks: 2,
                })
            );
            assert_eq!(
                file_manager.create_file(&mut process, 'D', 1),
                Some(DiskSegment {
                    offset: 1,
                    num_blocks: 1,
                })
            );
            assert_eq!(file_manager.create_file(&mut process, 'E', 1), None)
        }
    }

    mod delete_file {
        use super::*;

        #[test]
        fn test_delete_file_not_found() {
            let mut file_manager = create_test_file_manager();
            let real_time_process = create_process_mock(0);
            assert!(file_manager.delete_file(&real_time_process, 'A').is_ok());
            assert_eq!(
                file_manager.delete_file(&real_time_process, 'C'),
                Err(DeleteFileError::NotFound)
            );
        }

        #[test]
        fn test_delete_file_auth() {
            let mut file_manager = create_test_file_manager();
            let real_time_process = create_process_mock(0);
            let mut user_process = create_process_mock(1);
            assert!(file_manager.delete_file(&real_time_process, 'A').is_ok());
            assert_eq!(
                file_manager.delete_file(&user_process, 'B'),
                Err(DeleteFileError::Unauthorized)
            );
            assert!(file_manager
                .create_file(&mut user_process, 'C', 1)
                .is_some());
            assert_eq!(file_manager.delete_file(&user_process, 'C'), Ok(()));
        }

        #[test]
        fn test_delete_file_free_segment() {
            let mut file_manager = create_test_file_manager();
            let mut real_time_process = create_process_mock(0);
            assert!(file_manager.delete_file(&real_time_process, 'B').is_ok());
            assert!(file_manager
                .create_file(&mut real_time_process, 'C', 5)
                .is_some());
        }
    }
}
