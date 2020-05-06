use std::sync::mpsc;

use crate::Error;
use crate::InterpreterCommand;
use crate::InterpreterCommandResult;

pub struct EventLoopHandle {
    interpreter_command_sender: mpsc::Sender<InterpreterCommand>,
    interpreter_command_result_receiver:
        mpsc::Receiver<InterpreterCommandResult>,
}

impl EventLoopHandle {
    pub fn new(
        interpreter_command_sender: mpsc::Sender<InterpreterCommand>,
        interpreter_command_result_receiver: mpsc::Receiver<
            InterpreterCommandResult,
        >,
    ) -> EventLoopHandle {
        EventLoopHandle {
            interpreter_command_sender,
            interpreter_command_result_receiver,
        }
    }

    pub fn send_command(&self, command: InterpreterCommand) -> Result<(), ()> {
        match self.interpreter_command_sender.send(command) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn receive_result(&self) -> Result<InterpreterCommandResult, ()> {
        match self.interpreter_command_result_receiver.recv() {
            Ok(result) => Ok(result),
            Err(_) => Err(()),
        }
    }
}
