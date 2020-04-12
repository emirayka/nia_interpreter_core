use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;

use crate::keyboard_remapper::keyboard_id::KeyboardId;
use crate::keyboard_remapper::key_id::KeyId;
use crate::keyboard_remapper::keyboard_listener_unifier::KeyboardListenerUnifier;
use crate::keyboard_remapper::key_chord::KeyChord;
use crate::keyboard_remapper::keyboard_event::KeyboardEvent;
use crate::keyboard_remapper::KeyboardEventType;

fn toggle_modifier(
    map: &mut HashMap<(KeyboardId, KeyId), bool>,
    keyboard_event: &KeyboardEvent
) -> bool{
    let key_event = (
        keyboard_event.get_keyboard_id(),
        keyboard_event.get_key_id()
    );

    if map.contains_key(&key_event) {
        let reference = map.get_mut(&key_event).unwrap();

        *reference = !(*reference);
        true
    } else {
        false
    }
}

fn construct_key_chord(map: &HashMap<(KeyboardId, KeyId), bool>, event: KeyboardEvent) -> KeyChord {
    let modifier_keys = map.iter()
        .filter(|(_, pressed)| **pressed)
        .map(|(key, _)| key.clone())
        .collect();
    let ordinary_key = (
        event.get_keyboard_id(),
        event.get_key_id(),
    );

    KeyChord::new(
        modifier_keys,
        ordinary_key
    )
}

#[derive()]
pub struct KeyChordProducer {
    listener: KeyboardListenerUnifier,
    modifier_keys: Vec<(KeyboardId, KeyId)>,
}

impl KeyChordProducer {
    pub fn new(
        listener: KeyboardListenerUnifier,
        modifier_keys: Vec<(KeyboardId, KeyId)>,
    ) -> KeyChordProducer {
        KeyChordProducer {
            listener,
            modifier_keys,
        }
    }

    pub fn start_listening(&self) -> mpsc::Receiver<KeyChord> {
        let modifier_keys = self.modifier_keys.clone();

        let (
            tx,
            rx
        ) = mpsc::channel();

        let (
            tx2,
            rx2
        ) = mpsc::channel();

        self.listener.start_listening(tx);

        thread::spawn(move || {
            let modifier_keys = modifier_keys;

            let mut modifier_map = HashMap::new();

            for modifier_key in modifier_keys {
                modifier_map.insert(modifier_key, false);
            }

            loop {
                let keyboard_event = rx.recv().unwrap();
                let is_modifier_event = toggle_modifier(&mut modifier_map, &keyboard_event);

                if keyboard_event.get_event_type() == KeyboardEventType::RELEASED {
                    continue;
                }

                if !is_modifier_event {
                    let key_chord = construct_key_chord(
                        &modifier_map,
                        keyboard_event
                    );

                    tx2.send(key_chord);
                }
            }
        });

        rx2
    }
}