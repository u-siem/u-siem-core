use std::time::Duration;

use crossbeam_channel::Sender;
use usiem::prelude::{kernel_message::KernelMessager, NotificationLevel, SiemMessage};

#[cfg(not(lib_build))]
#[macro_use]
extern crate usiem;

fn initialize_component_logger(sender: Sender<SiemMessage>) {
    let mut msngr = KernelMessager::new(123, format!("Component001"), sender);
    msngr.set_level(NotificationLevel::Trace);
    usiem::logging::initialize_component_logger(msngr);
}

#[test]
fn test_all_logging() {
    let (sender, receiver) = crossbeam_channel::bounded(10);
    initialize_component_logger(sender.clone());
    error!("This is log name: {}", "PEPE");
    warn!("This is log name: {}", "PEPE");
    info!("This is log name: {}", "PEPE");
    debug!("This is log name: {}", "PEPE");
    trace!("This is log name: {}", "PEPE");
    for i in 1..6 {
        let msg = receiver
            .recv_timeout(Duration::from_millis(1000))
            .expect("Should send a message");
        match msg {
            SiemMessage::Notification(ntf) => {
                assert_eq!("This is log name: PEPE", ntf.log);
                assert_eq!(i, ntf.level as usize);
                assert_eq!(123, ntf.component);
                assert_eq!("Component001", ntf.component_name);
                assert!(ntf.timestamp > 0);
            }
            _ => unreachable!("Cannot be other thing"),
        }
    }
}
