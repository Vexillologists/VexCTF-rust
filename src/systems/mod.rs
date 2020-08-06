use std::panic;

pub mod context;
pub mod crypto;
pub mod errors;
pub mod logging;
pub mod session;
pub mod webserver;

use log::error;

pub fn init() {
    logging::setup_logger().expect("Failed to setup logger");
    panic::set_hook(Box::new(|panic_info| {
        if let Some(message) = panic_info.message() {
            if let Some(location) = panic_info.location() {
                error!(
                    "Panicked at {}/{}:{}: {}",
                    location.file(),
                    location.line(),
                    location.column(),
                    message
                );
            } else {
                error!("Panicked: {}", message);
            }
        } else if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
            if let Some(location) = panic_info.location() {
                error!(
                    "Panicked at {}/{}:{}: {}",
                    location.file(),
                    location.line(),
                    location.column(),
                    message
                );
            } else {
                error!("Panicked: {}", message);
            }
        } else {
            error!("Panicked and could not get message");
        }
    }));
    dotenv::dotenv().expect("Failed to initialize dotenv");
    webserver::init();
}
