use nia_events::Worker;
use nia_events::WorkerHandle;

use crate::interpreter::Error;

pub struct NiaWorker {}

impl NiaWorker {
    pub fn new() -> NiaWorker {
        NiaWorker {}
    }

    pub fn start_sending(self) -> Result<WorkerHandle, Error> {
        let worker = Worker::new();

        let worker_handle = worker.start_working().map_err(|_| {
            Error::generic_execution_error("Cannot instantiate worker.")
        })?;

        Ok(worker_handle)
    }
}
