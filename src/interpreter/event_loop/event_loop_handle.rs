use std::sync::mpsc;

use crate::Error;
use crate::NiaInterpreterCommand;
use crate::NiaInterpreterCommandResult;

pub struct EventLoopHandle {
    interpreter_command_sender: mpsc::Sender<NiaInterpreterCommand>,
    interpreter_command_result_receiver:
        mpsc::Receiver<NiaInterpreterCommandResult>,
}

impl EventLoopHandle {
    pub fn new(
        interpreter_command_sender: mpsc::Sender<NiaInterpreterCommand>,
        interpreter_command_result_receiver: mpsc::Receiver<
            NiaInterpreterCommandResult,
        >,
    ) -> EventLoopHandle {
        EventLoopHandle {
            interpreter_command_sender,
            interpreter_command_result_receiver,
        }
    }

    pub fn send_command(
        &self,
        command: NiaInterpreterCommand,
    ) -> Result<(), ()> {
        match self.interpreter_command_sender.send(command) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn receive_result(&self) -> Result<NiaInterpreterCommandResult, ()> {
        match self.interpreter_command_result_receiver.recv() {
            Ok(result) => Ok(result),
            Err(_) => Err(()),
        }
    }
}
