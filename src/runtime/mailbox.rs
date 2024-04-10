use crossbeam_channel::{Receiver, Sender};

use crate::components::common::SiemMessage;

pub struct Mailbox {
    pub tx : Sender<SiemMessage>,
    pub rx : Receiver<SiemMessage>
}

impl Default for Mailbox {
    fn default() -> Self {
        let (tx, rx) = crossbeam_channel::bounded(128);
        Mailbox {tx, rx}
    }
}
