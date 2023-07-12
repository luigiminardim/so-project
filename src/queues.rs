use std::collections::VecDeque;

use crate::process::Process;

pub struct ExecutionContext {
    pub process: Process,
    start_time: usize,
}

pub struct ProcessManager {
    pub execution: Option<ExecutionContext>,
    queues: Vec<VecDeque<Process>>,
}

impl ProcessManager {
    pub fn new() -> ProcessManager {
        ProcessManager {
            execution: None,
            queues: (0..3).map(|_| VecDeque::new()).collect(),
        }
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

    pub fn block_current_process(&mut self, timestamp: usize) -> Option<Process> {
        let process = self.execution.take()?.process;
        self.fill_executing_context(timestamp);
        Some(process)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_process_mock(priority: usize) -> Process {
        Process::new(priority)
    }

    #[test]
    fn test_add_process() {
        let mut process_manager = ProcessManager::new();
        let process = create_process_mock(0);
        assert!(process_manager.execution.is_none());
        process_manager.add_process(process, 0);
        assert!(process_manager.execution.is_some());
    }
}
