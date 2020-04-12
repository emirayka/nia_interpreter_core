extern crate evdev_rs;
extern crate rand;
extern crate nom;
extern crate state_machine;

use crate::keyboard_remapper::KeyChord;
use evdev_rs::enums::EV_KEY;

mod keyboard_remapper;
pub mod parser;
pub mod interpreter;

mod repl;

// todo: implement reference counting
// todo: Add better error handling
// todo: Write stdlib
// todo: Implement keyboard listening
// todo: binary plugins
// todo: ordinary plugins
// todo: file system
// todo: threading
// todo: implement constant checking, and move checking setting nil errors to interpreter itself

fn main() -> Result<(), std::io::Error> {
    repl::run()?;

    // let mut keyboard_listener_unifier = keyboard_remapper::KeyboardListenerUnifier::new();
    //
    // let keyboard_listener_1 = keyboard_remapper::KeyboardListener::new(
    //     keyboard_remapper::KeyboardId::new(0),
    //     String::from("/dev/input/event6")
    // );
    //
    // let keyboard_listener_2 = keyboard_remapper::KeyboardListener::new(
    //     keyboard_remapper::KeyboardId::new(1),
    //     String::from("/dev/input/event14")
    // );
    //
    // keyboard_listener_unifier.add_keyboard_listener(keyboard_listener_1);
    // keyboard_listener_unifier.add_keyboard_listener(keyboard_listener_2);
    //
    // let key_chord_producer = keyboard_remapper::KeyChordProducer::new(
    //     keyboard_listener_unifier,
    //     vec!()
    // );
    //
    // let receiver = key_chord_producer.start_listening();
    //
    // let mut sm = state_machine::StateMachine::new();
    // let keyboard_id = keyboard_remapper::KeyboardId::new(0);
    // let key_q = keyboard_remapper::KeyId::from_ev_key(EV_KEY::KEY_Q);
    // let key_w = keyboard_remapper::KeyId::from_ev_key(EV_KEY::KEY_W);
    // let key_f = keyboard_remapper::KeyId::from_ev_key(EV_KEY::KEY_F);
    //
    // sm.add(vec!(
    //     KeyChord::new(
    //         vec!(),
    //         (keyboard_id, key_q)
    //     ),
    //     KeyChord::new(
    //         vec!(),
    //         (keyboard_id, key_w)
    //     ),
    //     KeyChord::new(
    //         vec!(),
    //         (keyboard_id, key_f)
    //     ),
    // ), 1);
    //
    // loop {
    //     let key_chord = receiver.recv().unwrap();
    //     println!("{:?}", key_chord);
    //
    //     match sm.excite(&key_chord) {
    //         Some(action) => println!("{}", action),
    //         _ => {}
    //     }
    // }

    Ok(())
}
