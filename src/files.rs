use std::collections::HashMap;

use crate::process::Process;
use crate::structures::segment_list::{Segment, SegmentList};

pub struct FileManager {
    free_segments: SegmentList,
    alloc_map: HashMap<char, Segment>,
}

#[derive(Debug, PartialEq)]
pub enum DeleteFileError {
    NotFound,
    Unauthorized,
}

impl FileManager {
    pub fn new(num_blocks: usize, alloc_disk_blocks: Vec<(char, Segment)>) -> FileManager {
        let initial_segment = Segment {
            offset: 0,
            length: num_blocks,
        };
        let mut free_segments = SegmentList::new(vec![initial_segment]);
        for (_, alloc_segment) in alloc_disk_blocks.iter() {
            free_segments.alloc_segment(alloc_segment);
        }
        let alloc_map = alloc_disk_blocks.into_iter().collect();
        FileManager {
            free_segments,
            alloc_map,
        }
    }

    pub fn create_file(
        &mut self,
        process: &mut Process,
        file_name: char,
        num_blocks: usize,
    ) -> Option<Segment> {
        let alloc_segment = self.free_segments.alloc(num_blocks)?;
        self.alloc_map.insert(file_name, alloc_segment.clone());
        process.software_context.files_created.push(file_name);
        Some(alloc_segment)
    }

    pub fn delete_file(
        &mut self,
        process: &Process,
        file_name: char,
    ) -> Result<(), DeleteFileError> {
        let is_real_time_process = process.software_context.priority == 0;
        let process_created_file = process.software_context.files_created.contains(&file_name);
        let is_authorized = is_real_time_process || process_created_file;
        if !is_authorized {
            return Err(DeleteFileError::Unauthorized);
        }
        let disk_segment = self
            .alloc_map
            .remove(&file_name)
            .ok_or(DeleteFileError::NotFound)?;
        self.free_segments.free(disk_segment);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_process_mock(priority: usize) -> Process {
        Process::new(
            priority,
            0,
            false,
            false,
            false,
            false,
            vec![],
            Segment {
                offset: 0,
                length: 0,
            },
        )
    }

    mod create_file {
        use super::*;

        #[test]
        fn create_file_success() {
            let mut file_manager = FileManager::new(6, vec![]);
            let mut process: Process = create_process_mock(0);
            let result = file_manager.create_file(&mut process, 'A', 3);
            assert_eq!(
                result,
                Some(Segment {
                    offset: 0,
                    length: 3,
                })
            );
            assert!(process.software_context.files_created.contains(&'A'));
        }

        #[test]
        fn test_create_file_no_space() {
            let mut file_manager = FileManager::new(6, vec![]);
            let mut process = create_process_mock(0);
            assert_eq!(file_manager.create_file(&mut process, 'A', 7), None);
        }
    }

    mod delete_file {
        use super::*;

        #[test]
        fn file_not_found() {
            let mut file_manager = FileManager::new(6, vec![]);
            let real_time_process = create_process_mock(0);
            let result = file_manager.delete_file(&real_time_process, 'A');
            assert_eq!(result, Err(DeleteFileError::NotFound));
        }

        #[test]
        fn user_process_unauthorized() {
            let mut file_manager = FileManager::new(
                6,
                vec![(
                    'A',
                    Segment {
                        offset: 0,
                        length: 3,
                    },
                )],
            );
            let user_process = create_process_mock(1);
            let result = file_manager.delete_file(&user_process, 'A');
            assert_eq!(result, Err(DeleteFileError::Unauthorized));
        }

        #[test]
        fn user_process_authorized() {
            let mut file_manager = FileManager::new(6, vec![]);
            let mut user_process = create_process_mock(1);
            assert!(file_manager
                .create_file(&mut user_process, 'A', 3)
                .is_some());
            assert!(file_manager.delete_file(&user_process, 'A').is_ok());
        }

        #[test]
        fn real_time_process_always_authorized() {
            let mut file_manager = FileManager::new(
                6,
                vec![(
                    'A',
                    Segment {
                        offset: 0,
                        length: 3,
                    },
                )],
            );
            let real_time_process = create_process_mock(0);
            assert!(file_manager.delete_file(&real_time_process, 'A').is_ok());
        }
    }

    mod strong_test {
        use super::*;

        #[test]
        fn adds_and_deletes() {
            let mut file_manager = FileManager::new(10, {
                vec![
                    (
                        'X',
                        Segment {
                            offset: 0,
                            length: 2,
                        },
                    ),
                    (
                        'Y',
                        Segment {
                            offset: 3,
                            length: 1,
                        },
                    ),
                    (
                        'Z',
                        Segment {
                            offset: 5,
                            length: 3,
                        },
                    ),
                ]
            });
            let mut process_vec = vec![create_process_mock(0)];
            assert!(file_manager.delete_file(&process_vec[0], 'X').is_ok());
            let result = file_manager.create_file(&mut process_vec[0], 'D', 3);
            assert_eq!(
                result,
                Some(Segment {
                    offset: 0,
                    length: 3,
                })
            );
            assert!(process_vec[0].software_context.files_created.contains(&'D'));
        }
    }
}
