use env_logger::{self, fmt::Color, Builder};
use log::Level;
use std::io::Write;
pub struct Logger;

impl Logger {
    pub fn init() {
        let mut log_builder = Builder::from_env(env_logger::Env::new().default_filter_or("info"));
        log_builder
            .format(|buf, record| {
                let mut level_style = buf.style();
                let mut time_style = buf.style();
                let mut args_style = buf.style();

                level_style
                    .set_color(match record.level() {
                        Level::Error => Color::Red,
                        Level::Info => Color::Cyan,
                        Level::Debug => Color::Rgb(139, 0, 0),
                        _ => Color::Yellow,
                    })
                    .set_bold(true);
                time_style.set_color(Color::Rgb(255, 165, 0)).set_bold(true);

                args_style.set_color(Color::White).set_bold(true);

                writeln!(
                    buf,
                    "[{}] -> {} : {}",
                    time_style.value(buf.timestamp()),
                    level_style.value(record.level()),
                    args_style.value(record.args())
                )
            })
            .init();
    }
}
