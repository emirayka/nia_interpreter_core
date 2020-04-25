use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::sync::mpsc::{TryRecvError, Sender};

use nia_events::{KeyChordPart, EventListener, CommandSender};
use nia_events::Event;
use nia_events::EventListenerSettingsBuilder;
use nia_events::KeyboardId;
use nia_events::Command;
use nia_events::KeyChord;

use nia_state_machine::StateMachineResult;

use crate::interpreter::{Interpreter, Value, Action, Error};

use crate::interpreter::library;
use std::convert::TryFrom;

pub struct NiaCommandSender {
}

impl NiaCommandSender {
    pub fn new() -> NiaCommandSender {
        NiaCommandSender {
        }
    }

    pub fn start_sending(
        &self,
    ) -> (mpsc::Sender<Command>, mpsc::Sender<()>) {
        let command_sender = CommandSender::new();
        let (cmd_sender, event_stopper) = command_sender.start_sending();

        (cmd_sender, event_stopper)
    }
}
