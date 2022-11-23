use std::thread::JoinHandle;

use crossbeam_channel::{SendError, Sender};

use crate::components::{
    command::{SiemCommandCall, SiemCommandHeader},
    common::SiemMessage,
    SiemComponent,
};

/// Simplify the process of testing components.
/// This lets you execute some actions like send SiemMessages before stopping a component
pub fn do_before_stoping_component<F>(
    component: &Box<dyn SiemComponent>,
    action: F,
) -> JoinHandle<Result<(), SendError<SiemMessage>>>
where
    F: FnOnce() -> () + Send + 'static,
{
    let sender: Sender<SiemMessage> = component.local_channel();
    let comp_name = component.name().to_string();
    std::thread::spawn(move || {
        action();
        sender.send(SiemMessage::Command(
            SiemCommandHeader {
                comp_id: 0,
                comm_id: 0,
                user: String::from("kernel"),
            },
            SiemCommandCall::STOP_COMPONENT(comp_name),
        ))
    })
}
