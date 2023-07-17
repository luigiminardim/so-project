mod resource_mutex;

use crate::process::Process;

pub use self::resource_mutex::Resource;
use self::resource_mutex::ResourceMutex;

pub struct ResourceManager {
    resource_mutex_vec: Vec<ResourceMutex>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resource_mutex_vec: {
                let mut vec = Vec::new();
                vec.insert(
                    Resource::Scanner as usize,
                    ResourceMutex::new(vec![Resource::Scanner; 1]),
                );
                vec.insert(
                    Resource::Printer as usize,
                    ResourceMutex::new(vec![Resource::Printer; 2]),
                );
                vec.insert(
                    Resource::Modem as usize,
                    ResourceMutex::new(vec![Resource::Modem; 1]),
                );
                vec.insert(
                    Resource::SataDevice as usize,
                    ResourceMutex::new(vec![Resource::SataDevice; 2]),
                );
                vec
            },
        }
    }

    pub fn request(&mut self, process: Process, resource: Resource) -> Option<Process> {
        let process_id = process.software_context.id;
        let resource_mutex = &mut self.resource_mutex_vec[resource as usize];
        match resource_mutex.request(process) {
            None => {
                println!(
                    "Process {} blocked waiting for resource {:?}\n",
                    process_id, resource
                );
                None
            }
            Some((resource, mut process)) => {
                println!("Process {} allocated resource {:?}\n", process_id, resource);
                process.software_context.resources.push(resource);
                Some(process)
            }
        }
    }

    pub fn release_resources(&mut self, process: &mut Process) -> Vec<Process> {
        let mut unblocked_processes = Vec::new();
        for free_resource in process.software_context.resources.iter() {
            println!(
                "Process {} releasing resource {:?}",
                process.software_context.id, free_resource
            );
            match self.resource_mutex_vec[*free_resource as usize].release(*free_resource) {
                None => (),
                Some((resource, mut process)) => {
                    println!("Process {} unblocked", process.software_context.id);
                    process.software_context.resources.push(resource);
                    unblocked_processes.push(process);
                }
            }
        }
        process.software_context.resources.clear();
        println!("");
        unblocked_processes
    }
}

#[cfg(test)]
mod tests {
    use crate::structures::segment_list::Segment;

    use super::*;

    fn create_process_mock() -> Process {
        Process::new(
            0,
            1,
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

    mod request {
        use super::*;

        #[test]
        fn return_process_when_resource_is_available() {
            let mut resource_manager = ResourceManager::new();
            let process = create_process_mock();
            let result = resource_manager.request(process, Resource::Scanner);
            assert!(result.is_some());
        }

        #[test]
        fn return_none_when_resource_is_not_available() {
            let mut resource_manager = ResourceManager::new();
            let process = create_process_mock();
            let result = resource_manager.request(process, Resource::Scanner);
            assert!(result.is_some());
            let process = create_process_mock();
            let result = resource_manager.request(process, Resource::Scanner);
            assert!(result.is_none());
        }
    }

    mod release_resources {
        use super::*;

        #[test]
        fn return_unblocked_processes() {
            let mut resource_manager = ResourceManager::new();
            let greedy_process = create_process_mock();
            let greedy_process = resource_manager
                .request(greedy_process, Resource::Scanner)
                .unwrap();
            let mut greedy_process = resource_manager
                .request(greedy_process, Resource::Modem)
                .unwrap();
            let scanner_process = create_process_mock();
            resource_manager.request(scanner_process, Resource::Scanner);
            let modem_process = create_process_mock();
            resource_manager.request(modem_process, Resource::Modem);
            let unblocked_processes = resource_manager.release_resources(&mut greedy_process);
            assert_eq!(unblocked_processes.len(), 2);
        }
    }
}
