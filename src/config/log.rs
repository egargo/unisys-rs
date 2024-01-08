use crate::config::api::Config;
use chrono::Local;
use env_logger::{fmt::Color, Builder};
use std::io::Write;

impl Config {
    pub fn log() {
        Builder::new()
            .format(|buf, record| {
                let ist = Local::now();
                let mut level_style = buf.style();
                level_style.set_color(Color::Green).set_bold(true);

                writeln!(
                    buf,
                    "{} [{}] - {}",
                    ist.format("%Y-%m-%d %H:%M:%S"),
                    level_style.value(record.level()),
                    record.args()
                )
            })
            .filter(None, log::LevelFilter::Info)
            .init()
    }
}
