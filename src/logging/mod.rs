use std::{
    cell::RefCell,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::prelude::NotificationLevel;
use crate::runtime::channel::RuntimeChannel;
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
    pub static COMPONENT_LOGGER : RefCell<RuntimeChannel> = RefCell::new(RuntimeChannel::default());
}

/// Initializes the channel to communicate with the Kernel for the current thread/component.
pub fn initialize_component_logger(msngr: RuntimeChannel) {
    let _ = COMPONENT_LOGGER.with(|v| {
        let mut brw = v.borrow_mut();
        *brw = msngr;
        Ok::<(), ()>(())
    });
    // Wait for local_key_cell_methods
    //COMPONENT_LOGGER.replace(msngr);
}

/// Use for fast initialization of components during testing. With component ID "1234" and name "Dummy"
pub fn testing_component_logger_dummy() -> crossbeam_channel::Receiver<crate::prelude::SiemMessage>
{
    let (sender, receiver) = crossbeam_channel::unbounded();
    let msngr = RuntimeChannel::new(1234, "Dummy".to_string(), sender);
    initialize_component_logger(msngr);
    receiver
}

/// Use for fast initialization of components during testing
pub fn testing_component_logger(
    id: u64,
    name: &str,
) -> crossbeam_channel::Receiver<crate::prelude::SiemMessage> {
    let (sender, receiver) = crossbeam_channel::unbounded();
    let msngr = RuntimeChannel::new(id, name.to_string(), sender);
    initialize_component_logger(msngr);
    receiver
}
