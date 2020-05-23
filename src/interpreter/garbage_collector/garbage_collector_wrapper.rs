use std::time::SystemTime;
use std::time::{Duration, UNIX_EPOCH};

use crate::collect_garbage;
use crate::Error;
use crate::Interpreter;

fn get_current_time() -> Duration {
    let current_time = SystemTime::now();

    current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

pub struct GarbageCollectorWrapper {
    period: Duration,
    time_for_garbage_collection: Duration,
}

impl GarbageCollectorWrapper {
    pub fn new(period_ms: u64) -> GarbageCollectorWrapper {
        let period = Duration::from_millis(period_ms);

        let current_time = get_current_time();
        let time_for_garbage_collection = current_time + period;

        GarbageCollectorWrapper {
            period,
            time_for_garbage_collection,
        }
    }

    pub fn probably_collect(
        &mut self,
        interpreter: &mut Interpreter,
    ) -> Result<bool, Error> {
        let current_time = get_current_time();

        if current_time >= self.time_for_garbage_collection {
            self.time_for_garbage_collection = current_time + self.period;

            let time_before_collection = get_current_time();
            collect_garbage(interpreter)?;
            let time_after_collection = get_current_time();

            let diff = time_after_collection - time_before_collection;

            println!(
                "Collected garbage in {}.{} ms.",
                diff.as_millis(),
                diff.as_micros() % 1000
            );

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
