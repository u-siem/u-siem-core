use std::borrow::Cow;
use crossbeam_channel::{Sender, Receiver};
use super::events::SiemLog;
pub mod common;
pub mod dataset;
use common::{SiemMessage, SiemComponentStateStorage, SiemComponentCapabilities};
use std::boxed::Box;

pub trait SiemComponent {
    fn name(&self) -> Cow<'static, str>{
        return Cow::Borrowed("SiemComponent")
    }
    /// Get the channel to this component
    fn local_channel(&self) -> Sender<SiemMessage>;
    /// Sets the channel of this component. It's the kernel who sets the channel
    fn set_log_channel(&mut self, sender : Sender<SiemLog>, receiver : Receiver<SiemLog>);

    /// Sets the channel to communicate with the kernel.
    fn set_kernel_sender(&mut self, sender : Sender<SiemMessage>);

    /// Execute the logic of this component in an infinite loop. Must be stopped using Commands sent using the channel.
    fn run(&mut self);

    /// Allow to store information about this component like the state or conigurations.
    fn set_storage(&mut self, conn : Box<dyn SiemComponentStateStorage>);

    /// Capabilities and actions that can be performed by this component
    fn capabilities(&self) -> SiemComponentCapabilities;
}
