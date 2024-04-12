use std::thread::JoinHandle;

use crossbeam_channel::{SendError, Sender};

use crate::{
    components::{
        command::SiemCommand,
        common::SiemMessage,
        SiemComponent,
    },
    runtime::address::Mailed,
};
pub mod parsers;

/// Simplify the process of testing components.
/// Allows executing some actions like send SiemMessages before stopping a component
pub fn do_before_stoping_component<F>(
    component: &Box<dyn SiemComponent>,
    action: F,
) -> JoinHandle<Result<(), SendError<SiemMessage>>>
where
    F: FnOnce() + Send + 'static,
{
    let sender: Sender<SiemMessage> = component.local_channel();
    let comp_name = component.name().to_string();
    std::thread::spawn(move || {
        action();
        sender.send(SiemMessage::Command(Mailed::new(
            0,0,
            SiemCommand::STOP_COMPONENT(comp_name),
        )))
    })
}
