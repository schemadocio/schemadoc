mod services;

mod google_chats;
mod slack;
mod utils;

pub use services::{get_alerts_info, send_alert};
