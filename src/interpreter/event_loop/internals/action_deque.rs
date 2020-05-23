use std::collections::VecDeque;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::Action;
use std::env::current_dir;

fn get_current_time() -> Duration {
    let current_time = SystemTime::now();

    current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

pub struct ActionDeque {
    actions: VecDeque<Action>,
    last_checked_time: Duration,
}

impl ActionDeque {
    pub fn new() -> ActionDeque {
        let last_time = get_current_time();

        ActionDeque {
            actions: VecDeque::new(),
            last_checked_time: last_time,
        }
    }

    pub fn push_action(&mut self, action: Action) {
        self.actions.push_back(action);
    }

    pub fn push_actions(&mut self, actions: Vec<Action>) {
        for action in actions {
            self.actions.push_back(action);
        }
    }

    pub fn take_action(&mut self) -> Option<Action> {
        let current_time = get_current_time();
        let diff = current_time - self.last_checked_time;

        self.last_checked_time = current_time;

        let take = match self.actions.front_mut() {
            Some(Action::Wait(remaining_ms)) => {
                let diff_as_ms = diff.as_millis();

                if diff_as_ms >= (*remaining_ms as u128) {
                    true
                } else {
                    *remaining_ms -= diff_as_ms as i32;
                    false
                }
            }
            Some(_) => true,
            None => false,
        };

        if take {
            self.actions.pop_front()
        } else {
            None
        }
    }
}
