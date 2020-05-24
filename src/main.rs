pub mod utils;

pub mod interpreter;
pub mod repl;

pub use interpreter::*;

use chrono::Local;
use env_logger::Builder;
use log::debug;
use log::info;
use log::warn;
use log::LevelFilter;
use std::io::Write;

// todo: Add better error handling
// todo: binary plugins
// todo: file system
// todo: threading

fn main() -> std::io::Result<()> {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    println!(
        "{}",
        KeyChord::new(
            vec!(Key::DeviceKey(DeviceKey::new(0, 58))),
            Key::DeviceKey(DeviceKey::new(0, 2))
        ) == KeyChord::new(
            vec!(Key::LoneKey(LoneKey::new(58))),
            Key::LoneKey(LoneKey::new(2))
        )
    );

    repl::run()?;

    Ok(())
}
