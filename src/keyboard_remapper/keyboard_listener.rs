use std::fs::File;
use std::thread;
use std::sync::mpsc;

use crate::keyboard_remapper::keyboard_id::KeyboardId;
use crate::keyboard_remapper::keyboard_event::{KeyboardEvent, KeyboardEventType};
use crate::keyboard_remapper::key_id::KeyId;
use evdev_rs::enums::EventCode;

pub struct KeyboardListener {
    keyboard_id: KeyboardId,
    device_path: String,
}

impl KeyboardListener {
    pub fn new(keyboard_id: KeyboardId, device_path: String) -> KeyboardListener {
        KeyboardListener {
            keyboard_id,
            device_path,
        }
    }

    pub fn start_listening(&self, sender: mpsc::Sender<KeyboardEvent>) {
        let sender = sender;
        let device_path = self.device_path.clone();
        let keyboard_id = self.keyboard_id;

        thread::spawn(move || {
            let f1 = File::open(device_path).unwrap();
            let mut device = evdev_rs::device::Device::new().unwrap();

            device.set_fd(f1);
            device.grab(evdev_rs::GrabMode::Grab);

            let flags = evdev_rs::ReadFlag::NORMAL | evdev_rs::ReadFlag::BLOCKING;

            loop {
                match device.next_event(flags) {
                    Ok((read_status, event)) => {
                        match read_status {
                            evdev_rs::ReadStatus::Sync => {
                            },
                            evdev_rs::ReadStatus::Success => {
                                match event.event_type {
                                    evdev_rs::enums::EventType::EV_KEY => {
                                        let keyboard_id = keyboard_id;

                                        let key_id = if let EventCode::EV_KEY(ev_key) = event.event_code {
                                            KeyId::from_ev_key(ev_key)
                                        } else {
                                            continue;
                                        };

                                        let event_type = KeyboardEventType::from_value(
                                            event.value
                                        );

                                        let keyboard_event = KeyboardEvent::new(
                                            keyboard_id,
                                            key_id,
                                            event_type
                                        );

                                        sender.send(keyboard_event);
                                    },
                                    _ => {}
                                }
                            },
                        }
                    },
                    Err(_) => {
                        panic!();
                    }
                }
            }
        });
    }
}
