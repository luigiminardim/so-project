use std::collections::VecDeque;

use crate::process::Process;

pub struct ExecutionContext {
    process: Process,
    start_time: usize,
}

pub struct ProcessManager {
    execution: Option<ExecutionContext>,
    queues: Vec<VecDeque<Process>>,
}

impl ProcessManager {
    pub fn new() -> ProcessManager {
        ProcessManager {
            execution: None,
            queues: (0..3).map(|_| VecDeque::new()).collect(),
        }
    }

    pub fn get_current_process(&mut self) -> Option<&mut Process> {
        self.execution
            .as_mut()
            .map(|execution_context| &mut execution_context.process)
    }

    fn fill_executing_context(&mut self, timestamp: usize) {
        if let None = self.execution {
            let next_process_option = self.queues.iter_mut().find_map(|queue| queue.pop_front());
            if let Some(next_process) = next_process_option {
                self.execution = Some(ExecutionContext {
                    process: next_process,
                    start_time: timestamp,
                })
            }
        }
    }

    pub fn add_process(&mut self, process: Process, timestamp: usize) {
        let priority = process.software_context.priority;
        self.queues[priority].push_back(process);
        self.fill_executing_context(timestamp);
    }

    pub fn block_current_process(&mut self) -> Option<Process> {
        let process = self.execution.take()?.process;
        Some(process)
    }

    pub fn terminate_current_process(&mut self) -> Option<Process> {
        let process = self.execution.take()?.process;
        Some(process)
    }

    pub fn on_tick(&mut self, timestamp: usize) {
        const USER_PROCESS_QUANTUM: usize = 1;
        let should_change_context = match self.execution.as_ref() {
            None => true,
            Some(execution_context) => match execution_context.process.software_context.priority {
                0 => false,
                _ => timestamp - execution_context.start_time >= USER_PROCESS_QUANTUM,
            },
        };
        if should_change_context {
            match self.execution.take() {
                None => {}
                Some(execution_context) => {
                    let mut process = execution_context.process;
                    process.software_context.priority =
                        std::cmp::min(process.software_context.priority + 1, 3);
                    self.add_process(process, timestamp);
                }
            }
            self.fill_executing_context(timestamp)
        }
    }

    pub fn has_more_processes(&self) -> bool {
        self.queues.iter().any(|queue| !queue.is_empty()) || self.execution.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::structures::segment_list::Segment;

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

    #[test]
    fn test_add_process() {
        let mut process_manager = ProcessManager::new();
        let process = create_process_mock(0);
        assert!(process_manager.get_current_process().is_none());
        process_manager.add_process(process, 0);
        assert!(process_manager.get_current_process().is_some());
    }

    #[test]
    fn test_block_current_process() {
        let mut process_manager = ProcessManager::new();
        let process0 = create_process_mock(0);
        process_manager.add_process(process0, 0);
        let process1 = create_process_mock(1);
        process_manager.add_process(process1, 0);
        assert!(process_manager.get_current_process().is_some());
        process_manager.block_current_process();
        process_manager.on_tick(1);
        assert!(process_manager.get_current_process().is_some());
        process_manager.block_current_process();
        process_manager.on_tick(2);
        assert!(process_manager.get_current_process().is_none());
    }

    #[test]
    fn test_on_tick() {
        let mut process_manager = ProcessManager::new();
        let real_time_process = create_process_mock(0);
        let user_process = create_process_mock(1);
        process_manager.add_process(real_time_process, 0);
        process_manager.add_process(user_process, 0);
        assert!(
            process_manager
                .get_current_process()
                .unwrap()
                .software_context
                .priority
                == 0
        );
        process_manager.on_tick(1);
        assert!(
            process_manager
                .get_current_process()
                .unwrap()
                .software_context
                .priority
                == 0
        );

        let real_time_process = process_manager.block_current_process().unwrap();
        process_manager.on_tick(2);
        process_manager.add_process(real_time_process, 1);
        assert_eq!(
            process_manager
                .get_current_process()
                .unwrap()
                .software_context
                .priority,
            1
        );
        process_manager.on_tick(3);
        assert_eq!(
            process_manager
                .get_current_process()
                .unwrap()
                .software_context
                .priority,
            0
        );
    }
}
