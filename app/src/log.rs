use colored::Colorize;
use env_logger::{self, fmt::Color, Builder, Target::Stdout};
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
                if record.level().eq(&Level::Error) {
                    // Wrtie to stderr
                    eprintln!(
                        "[{}] {} -> {} : {} \n\t {}",
                        buf.timestamp().to_string().bold().yellow(),
                        record.target().to_string().bold(),
                        record.level().to_string().bold().red(),
                        record.args().to_string().bold(),
                        ("from: ".to_owned() + record.file().unwrap())
                            .bold()
                            .to_string(),
                    );
                } else {
                    // Write to stdout (default)
                    writeln!(
                        buf,
                        "[{}] -> {} : {}",
                        time_style.value(buf.timestamp()),
                        level_style.value(record.level()),
                        args_style.value(record.args())
                    )
                    .unwrap();
                }
                Ok(())
            })
            .target(Stdout)
            .init();
    }
}
