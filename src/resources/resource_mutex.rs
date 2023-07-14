use std::collections::VecDeque;

use crate::process::Process;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Resource {
    Scanner,
    Printer,
    Modem,
    SataDevice,
}

pub struct ResourceMutex {
    resources: Vec<Resource>,
    queue: VecDeque<Process>,
}

impl ResourceMutex {
    pub fn new(resources: Vec<Resource>) -> Self {
        ResourceMutex {
            resources,
            queue: VecDeque::new(),
        }
    }

    pub fn request(&mut self, process: Process) -> Option<(Resource, Process)> {
        let resource = self.resources.pop();
        match resource {
            Some(resource) => Some((resource, process)),
            None => {
                self.queue.push_back(process);
                None
            }
        }
    }

    pub fn release(&mut self, resource: Resource) -> Option<(Resource, Process)> {
        let process = self.queue.pop_front();
        match process {
            Some(process) => Some((resource, process)),
            None => {
                self.resources.push(resource);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_process_mock() -> Process {
        Process::new(1, 0, false, false, false, false, vec![])
    }

    mod request {
        use super::*;

        #[test]
        fn return_resource_and_process_when_resource_is_available() {
            let mut resource_mutex = ResourceMutex::new(vec![Resource::Scanner]);
            let process = create_process_mock();
            let result = resource_mutex.request(process);
            assert!(result.is_some());
        }

        #[test]
        fn return_none_when_resource_is_not_available() {
            let mut resource_mutex = ResourceMutex::new(vec![]);
            let process = create_process_mock();
            let result = resource_mutex.request(process);
            assert!(result.is_none());
        }

        #[test]
        fn multiple_resources() {
            let mut resource_mutex = ResourceMutex::new(vec![Resource::Scanner, Resource::Printer]);
            let process = create_process_mock();
            let (_, process) = resource_mutex.request(process).unwrap();
            let (_, process) = resource_mutex.request(process).unwrap();
            let fail_result = resource_mutex.request(process);
            assert!(fail_result.is_none());
        }
    }

    mod release {
        use super::*;

        #[test]
        fn return_resource_and_process_when_queue_is_not_empty() {
            let mut resource_mutex = ResourceMutex::new(vec![]);
            let process = create_process_mock();
            resource_mutex.request(process);
            let result = resource_mutex.release(Resource::Scanner);
            assert!(result.is_some());
        }

        #[test]
        fn return_none_when_queue_is_empty() {
            let mut resource_mutex = ResourceMutex::new(vec![]);
            let result = resource_mutex.release(Resource::Scanner);
            assert!(result.is_none());
        }
    }
}
