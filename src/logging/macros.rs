#[macro_export(local_inner_macros)]
macro_rules! error {
    // error!("a {} event", "log")
    ($($arg:tt)+) => (log!($crate::components::common::NotificationLevel::Error, $($arg)+))
}

#[macro_export(local_inner_macros)]
macro_rules! warn {
    // warn!("a {} event", "log")
    ($($arg:tt)+) => (log!($crate::components::common::NotificationLevel::Warn, $($arg)+))
}

#[macro_export(local_inner_macros)]
macro_rules! info {
    // info!("a {} event", "log")
    ($($arg:tt)+) => (log!($crate::components::common::NotificationLevel::Info, $($arg)+))
}

#[macro_export(local_inner_macros)]
macro_rules! debug {
    // debug!("a {} event", "log")
    ($($arg:tt)+) => (log!($crate::components::common::NotificationLevel::Debug, $($arg)+))
}

#[macro_export(local_inner_macros)]
macro_rules! trace {
    // trace!("a {} event", "log")
    ($($arg:tt)+) => (log!($crate::components::common::NotificationLevel::Trace, $($arg)+))
}

#[macro_export(local_inner_macros)]
macro_rules! log {
    // log!( Level::Info; "a {} event", "log");
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= $crate::components::common::NotificationLevel::Trace && lvl <= $crate::logging::max_level() {
            let _ = $crate::logging::COMPONENT_LOGGER.with(|v| {
                let msngr = v.borrow();
                msngr.log(std::format!($($arg)+), lvl);
                Ok::<(), ()>(())
            });
        }
    });
}
