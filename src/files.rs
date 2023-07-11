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
        let mut free_segments = vec![DiskSegment {
            offset: 0,
            num_blocks,
        }];
        for (_, disk_segment) in &alloc_disk_blocks {
            FileManager::remove_disk_segment(&mut free_segments, disk_segment);
        }
        let alloc_map = alloc_disk_blocks.into_iter().collect();
        FileManager {
            free_segments,
            alloc_map,
        }
    }

    fn remove_disk_segment(disk_segments: &mut Vec<DiskSegment>, to_remove: &DiskSegment) {
        let index = disk_segments.iter().position(|s| {
            s.offset <= to_remove.offset
                && s.offset + s.num_blocks >= to_remove.offset + to_remove.num_blocks
        });
        match index {
            None => {}
            Some(index) => {
                let left = DiskSegment {
                    offset: disk_segments[index].offset,
                    num_blocks: to_remove.offset - disk_segments[index].offset,
                };
                let right = DiskSegment {
                    offset: to_remove.offset + to_remove.num_blocks,
                    num_blocks: disk_segments[index].offset + disk_segments[index].num_blocks
                        - to_remove.offset
                        - to_remove.num_blocks,
                };
                if left.num_blocks == 0 && right.num_blocks == 0 {
                    disk_segments.remove(index);
                } else if left.num_blocks == 0 {
                    disk_segments[index] = right;
                } else if right.num_blocks == 0 {
                    disk_segments[index] = left;
                } else {
                    disk_segments[index] = left;
                    disk_segments.insert(index + 1, right);
                }
            }
        };
    }

    pub fn create_file(
        &mut self,
        process: &mut Process,
        file_name: char,
        num_blocks: usize,
    ) -> Option<DiskSegment> {
        let disk_segment = self.allocate(num_blocks)?;
        process.software_context.files_created.push(file_name);
        self.alloc_map.insert(file_name, disk_segment);
        Some(disk_segment)
    }

    fn allocate(&mut self, num_blocks: usize) -> Option<DiskSegment> {
        let index = self
            .free_segments
            .iter()
            .position(|segment| segment.num_blocks >= num_blocks)?;
        let offset = self.free_segments[index].offset;
        if num_blocks < self.free_segments[index].num_blocks {
            self.free_segments[index].offset += num_blocks;
            self.free_segments[index].num_blocks -= num_blocks;
        } else {
            self.free_segments.remove(index);
        }
        Some(DiskSegment { offset, num_blocks })
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

    mod new {
        use super::*;

        #[test]
        fn test_remove_disk_segment() {
            let mut disk_segments = vec![DiskSegment {
                offset: 0,
                num_blocks: 6,
            }];
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
            for (_, disk_segment) in &alloc_disk_blocks {
                FileManager::remove_disk_segment(&mut disk_segments, disk_segment);
            }
            assert_eq!(
                disk_segments,
                vec![
                    DiskSegment {
                        offset: 1,
                        num_blocks: 1,
                    },
                    DiskSegment {
                        offset: 4,
                        num_blocks: 2,
                    },
                ]
            );
        }
    }

    mod create_file {
        use super::*;

        #[test]
        fn test_allocate() {
            let mut file_manager = create_test_file_manager();
            assert_eq!(file_manager.allocate(3), None);
            assert_eq!(
                file_manager.allocate(2),
                Some(DiskSegment {
                    offset: 4,
                    num_blocks: 2,
                })
            );
            assert_eq!(
                file_manager.allocate(1),
                Some(DiskSegment {
                    offset: 1,
                    num_blocks: 1,
                })
            );
        }

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
            assert_eq!(file_manager.create_file(&mut process, 'E', 1), None,)
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
