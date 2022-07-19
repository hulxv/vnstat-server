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
                let is_systemd_style = std::env::var("SYSTEMD_EXEC_PID").is_ok();

                let mut level_style = buf.style();
                let mut time_style = buf.style();

                level_style.set_color(match record.level() {
                    Level::Error => Color::Red,
                    Level::Info => Color::Green,
                    Level::Debug => Color::Rgb(139, 0, 0),
                    _ => Color::Yellow,
                });
                time_style.set_color(Color::Rgb(255, 165, 0));
                if is_systemd_style {
                    writeln!(
                        buf,
                        "<{}>{}: {}",
                        match record.level() {
                            log::Level::Error => 3,
                            log::Level::Warn => 4,
                            log::Level::Info => 6,
                            log::Level::Debug => 7,
                            log::Level::Trace => 7,
                        },
                        record.target(),
                        record.args()
                    )
                    .unwrap();
                } else {
                    match record.level().eq(&Level::Error) {
                        // Write to stderr
                        true => eprintln!(
                            "{} {} : {} \n\t {} \n\t {}",
                            buf.timestamp().to_string().color(RGB::new(255, 165, 0)),
                            record.level().to_string().red(),
                            record.args().to_string().trim(),
                            format!("target: {}", record.target()),
                            format!(
                                "file:   {} ({})",
                                record.file().unwrap(),
                                record.line().unwrap().to_string().yellow()
                            ),
                        ),

                        // Write to stdout (default)
                        _ => writeln!(
                            buf,
                            "⌜{}⌟{ } : {}",
                            time_style.value(buf.timestamp()),
                            level_style.value(record.level()),
                            record.args().to_string().trim()
                        )
                        .unwrap(),
                    }
                }
                Ok(())
            })
            .target(Stdout)
            .init();
    }
}
