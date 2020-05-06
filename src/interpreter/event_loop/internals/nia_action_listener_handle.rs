use crate::{Action, Error};
use std::sync::mpsc;

pub struct NiaActionListenerHandle {
    action_receiver: mpsc::Receiver<Action>,
    stop_sender: mpsc::Sender<()>,
}

impl NiaActionListenerHandle {
    pub fn new(
        action_receiver: mpsc::Receiver<Action>,
        stop_sender: mpsc::Sender<()>,
    ) -> NiaActionListenerHandle {
        NiaActionListenerHandle {
            action_receiver,
            stop_sender,
        }
    }

    pub fn receive_action(&self) -> Result<Action, Error> {
        match self.action_receiver.recv() {
            Ok(action) => Ok(action),
            Err(_) => Error::generic_execution_error("").into(),
        }
    }

    pub fn try_receive_action(&self) -> Result<Action, mpsc::TryRecvError> {
        match self.action_receiver.try_recv() {
            Ok(action) => Ok(action),
            Err(try_recv_error) => Err(try_recv_error),
        }
    }

    pub fn stop(&self) -> Result<(), ()> {
        match self.stop_sender.send(()) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
