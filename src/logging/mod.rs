use std::{
    cell::RefCell,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::prelude::{kernel_message::KernelMessager, NotificationLevel};

#[macro_use]
pub mod macros;

static MAX_NOTIFY_LEVEL_FILTER: AtomicUsize = AtomicUsize::new(5);
//static NOTIFY_LEVEL_NAMES: [&str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

#[inline]
pub fn set_max_level(level: NotificationLevel) {
    MAX_NOTIFY_LEVEL_FILTER.store(level as usize, Ordering::Relaxed);
}

#[inline]
pub fn enabled_level(level: &NotificationLevel) -> bool {
    MAX_NOTIFY_LEVEL_FILTER.load(Ordering::Relaxed) >= (*level as usize)
}
#[inline]
pub fn max_level() -> NotificationLevel {
    unsafe { std::mem::transmute(MAX_NOTIFY_LEVEL_FILTER.load(Ordering::Relaxed)) }
}

thread_local! {
    pub static COMPONENT_LOGGER : RefCell<KernelMessager> = RefCell::new(KernelMessager::default());
}

pub fn initialize_component_logger(msngr: KernelMessager) {
    let _ = COMPONENT_LOGGER.with(|v| {
        let mut brw = v.borrow_mut();
        *brw = msngr;
        Ok::<(), ()>(())
    });
    // Wait for local_key_cell_methods
    //COMPONENT_LOGGER.replace(msngr);
}
