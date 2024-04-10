use crossbeam_channel::Sender;

use crate::components::common::SiemMessage;

use super::{actor::{Actor, ActorState}, mailbox::Mailbox};

/// An actor execution context.
pub struct Context {
    state : ActorState,
    mb: Mailbox
}

impl Context {
    pub fn new() -> Self {
        let mb = Mailbox::default();
        Self {
            state : ActorState::Stopped,
            mb
        }
    }

    pub fn run(self) -> Sender<SiemMessage> {
        let tx = self.mb.tx.clone();
        // Falta contexto de ejecucion
        tx
    }
}