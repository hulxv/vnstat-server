use colored::Colorize;
use env_logger::{self, fmt::Color, Builder, Target::Stdout};
use log::Level;
use std::io::Write;
use colorful::{Colorful, RGB};
pub struct Logger;

impl Logger {
    pub fn init() {
        let mut log_builder = Builder::from_env(env_logger::Env::new().default_filter_or("info"));
        log_builder
            .format(|buf, record| {
                let mut level_style = buf.style();
                let mut time_style = buf.style();

                level_style.set_color(match record.level() {
                    Level::Error => Color::Red,
                    Level::Info => Color::Green,
                    Level::Debug => Color::Rgb(139, 0, 0),
                    _ => Color::Yellow,
                });
                time_style.set_color(Color::Rgb(255, 165, 0));

                if record.level().eq(&Level::Error) {
                    // Write to stderr
                    eprintln!(
                        "⌜{}⌟ {} : {} \n\t {} \n\t {}",
                        buf.timestamp().to_string().bright_yellow(),
                        record.level().to_string().red(),
                        record.args().to_string().trim(),
                        format!("target: {}", record.target()),
                        format!(
                            "file:   {} ({})",
                            record.file().unwrap(),
                            record.line().unwrap()
                        ),
                    );
                } else {
                    // Write to stdout (default)
                    writeln!(
                        buf,
                        "⌜{}⌟ {} : {}",
                        time_style.value(buf.timestamp()),
                        level_style.value(record.level()),
                        record.args().to_string().trim()
                    )
                    .unwrap();
                }
                Ok(())
            })
            .target(Stdout)
            .init();
    }
}
