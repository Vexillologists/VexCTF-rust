use ansi_term::Color;
use ansi_term::Style;
use log::Level;

use std::fmt::Arguments;

use log::Record;

fn level_to_ansi(level: Level) -> String {
    let debug_style = Style::new().on(Color::Fixed(237)).fg(Color::White);
    let error_style = Style::new().on(Color::Red).fg(Color::Black);
    let info_style = Style::new().on(Color::Blue).fg(Color::Black);
    let warn_style = Style::new().on(Color::Yellow).fg(Color::Black);
    match level {
        Level::Error => error_style.paint(" ERR! ").to_string(),
        Level::Info => info_style.paint(" INFO ").to_string(),
        Level::Warn => warn_style.paint(" WARN ").to_string(),
        _ => debug_style.paint(" DEBG ").to_string(),
    }
}

fn message_to_ansi(level: Level, message: &str) -> String {
    let debug_format = Style::new().fg(Color::Fixed(247));
    let error_format = Style::new().fg(Color::Red);
    let warn_format = Style::new().fg(Color::Yellow);
    match level {
        Level::Error => error_format.paint(message).to_string(),
        Level::Info => message.to_string(),
        Level::Warn => warn_format.paint(message).to_string(),
        _ => debug_format.paint(message).to_string(),
    }
}

fn fix_message(message: &Arguments, record: &Record, use_ansi: bool) -> String {
    let mut message_fmt = format!("{}", message);

    if record.target().ends_with('_') {
        if use_ansi {
            message_fmt = format!("\t{} {}", Color::White.bold().paint("=>"), message_fmt);
        } else {
            message_fmt = format!("\t=> {}", message_fmt);
        }
    }

    message_fmt
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    let file_dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            let message = fix_message(message, record, false);
            out.finish(format_args!(
                "[{}] [{}]: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file("output.log")?);

    let debug_file_dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            let message = fix_message(message, record, false);
            out.finish(format_args!(
                "[{}] [{}]: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("debug.log")?);

    let out_dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            let message = fix_message(message, record, true);
            out.finish(format_args!(
                "{} {} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_to_ansi(record.level()),
                message_to_ansi(record.level(), &message)
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout());

    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(out_dispatch)
        .chain(file_dispatch)
        .chain(debug_file_dispatch)
        .apply()?;
    Ok(())
}
