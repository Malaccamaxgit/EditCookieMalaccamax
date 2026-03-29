//! Background service worker module

pub mod context_menus;
pub mod cookie_monitor;
pub mod devtools;

pub use context_menus::update_context_menus;
pub use cookie_monitor::init_cookie_monitor;
pub use devtools::init_devtools;
