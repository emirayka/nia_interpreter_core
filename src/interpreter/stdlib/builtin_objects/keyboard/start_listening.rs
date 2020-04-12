use std::fs::File;
use std::thread;
use std::sync::mpsc;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::environment::EnvironmentId;
use crate::interpreter::value::Value;
use crate::interpreter::error::Error;

use crate::interpreter::library;

use evdev_rs;

// pub fn start_loop() -> Result<(), Error> {
//     let (tx, rx) = mpsc::channel();
//
//     {
//         let tx = tx.clone();
//     }
//
//     {
//         let tx = tx.clone();
//         thread::spawn(move || {
//             let f1 = File::open("/dev/input/event14").unwrap();
//
//             let mut d1 = evdev_rs::device::Device::new().unwrap();
//
//             d1.set_fd(f1);
//             d1.grab(evdev_rs::GrabMode::Grab);
//
//             let flags = evdev_rs::ReadFlag::NORMAL | evdev_rs::ReadFlag::BLOCKING;
//
//             loop {
//                 match d1.next_event(flags) {
//                     Ok((read_status, event)) => {
//                         match read_status {
//                             evdev_rs::ReadStatus::Sync => {
//                             },
//                             evdev_rs::ReadStatus::Success => {
//                                 match event.event_type {
//                                     evdev_rs::enums::EventType::EV_KEY => {
//                                         tx.send((
//                                             2,
//                                             event.event_code,
//                                             event.value
//                                         ));
//                                     },
//                                     _ => {}
//                                 }
//                             },
//                         }
//                     },
//                     Err(_) => {
//                         panic!();
//                     }
//                 }
//             }
//         });
//     }
//
//     loop {
//         let result = rx.recv().unwrap();
//
//         let str = format!(
//             "Keyboard id: {}, event code: {}, event value: {}",
//             result.0,
//             result.1,
//             result.2,
//         );
//
//         println!("{}", str)
//     }
//
//     Ok(())
// }

pub fn start_loop(interpreter: &mut Interpreter) -> Result<(), Error> {
    let root_environment_id = interpreter.get_root_environment();
    let symbol_id_registered_keyboards = interpreter.intern("registered-keyboards");

    let registered_keyboards = interpreter.lookup_variable(
        root_environment_id,
        symbol_id_registered_keyboards,
    )?;

    library::check_value_is_cons(
        interpreter,
        registered_keyboards
    )?;

    let registered_keyboards = interpreter.list_to_vec(
        registered_keyboards.as_cons_id()
    )?;

    Ok(())
}

pub fn start_listening(
    interpreter: &mut Interpreter,
    _environment_id: EnvironmentId,
    values: Vec<Value>
) -> Result<Value, Error> {
    if values.len() != 0 {
        return interpreter.make_invalid_argument_count_error(
            "Built-in function `keyboard:start-listening' takes no arguments."
        ).into_result()
    }

    // start_loop()?;

    Ok(interpreter.intern_nil_symbol_value())
}

#[cfg(test)]
mod tests {
}
