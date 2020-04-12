use std::fs::File;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use evdev_rs;

pub fn start_loop(_interpreter: &mut Interpreter) -> Result<(), Error> {
    let f1 = File::open("/dev/input/event6").unwrap();
    let f2 = File::open("/dev/input/event14").unwrap();

    let mut d1 = evdev_rs::device::Device::new().unwrap();
    let mut d2 = evdev_rs::device::Device::new().unwrap();

    d1.set_fd(f1);
    d1.grab(evdev_rs::GrabMode::Grab);

    d2.set_fd(f2);
    d2.grab(evdev_rs::GrabMode::Grab);

    let flags = evdev_rs::ReadFlag::NORMAL | evdev_rs::ReadFlag::BLOCKING;

    loop {
        if d1.has_event_pending() {
             match d1.next_event(flags) {
                Ok((read_status, event)) => {
                    match read_status {
                        evdev_rs::ReadStatus::Sync => {
                        },
                        evdev_rs::ReadStatus::Success => {
                            match event.event_type {
                                evdev_rs::enums::EventType::EV_KEY => {
                                    println!(
                                        "Event: type {}, code {}, value {}",
                                        event.event_type,
                                        event.event_code,
                                        event.value
                                    );
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

        if d2.has_event_pending() {
            match d2.next_event(flags) {
                Ok((read_status, event)) => {
                    match read_status {
                        evdev_rs::ReadStatus::Sync => {
                        },
                        evdev_rs::ReadStatus::Success => {
                            match event.event_type {
                                evdev_rs::enums::EventType::EV_KEY => {
                                    println!(
                                        "Event: type {}, code {}, value {}",
                                        event.event_type,
                                        event.event_code,
                                        event.value
                                    );
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
    }

    Ok(())
}

pub fn start_listening(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 0 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `keyboard:start-listening' takes zero arguments exactly."
        ).into_result()
    }

    start_loop(interpreter)?;

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
}
