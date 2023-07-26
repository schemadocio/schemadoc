mod services;

mod google_chats;
mod slack;
mod utils;

pub use services::{get_own_alerts_info, get_deps_alerts_info, send_alert};
