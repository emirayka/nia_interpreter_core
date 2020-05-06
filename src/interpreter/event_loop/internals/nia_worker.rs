use nia_state_machine::StateMachineResult;

use nia_events::Worker;
use nia_events::WorkerHandle;

use crate::interpreter::Action;
use crate::interpreter::Error;
use crate::interpreter::Interpreter;
use crate::interpreter::Value;

use crate::interpreter::library;

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
