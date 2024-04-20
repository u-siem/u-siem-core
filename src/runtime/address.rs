use std::{cell::RefCell, time::{SystemTime, UNIX_EPOCH}};

use crossbeam_channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};

use crate::components::common::SiemMessage;

thread_local! {
    static ID_COUNTER : RefCell<u64> = RefCell::new(SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .subsec_nanos() as u64);
}

pub fn next_actor_id() -> u64 {
    ID_COUNTER.with_borrow_mut(|v| {
        // Pseudo random Actor ID. Its not required to be random but I dont like the IDs to be guessable
        let add = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as u64;
        *v = v.wrapping_add(add);
        let ret = *v;
        let add = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as u64;
        *v = v.wrapping_add(add);
        ret
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mailed<T> {
    pub msg : T,
    /// Msg ID. To track responses to commands or tasks
    track_id : u64,
    component_id : u64
}

impl<T> Mailed<T> {
    pub fn new(id : u64, component_id : u64, msg : T) -> Self {
        Self {
            track_id : id,
            component_id,
            msg
        }
    }

    pub fn id(&self) -> u64 {
        self.track_id
    }
    /// The tasks sets the component_id to the current actor ID
    pub(crate) fn mask(self, addr : &ActorAddrRecv) -> Self {
        Mailed::new(self.track_id, addr.id(), self.msg)
    }
    /// Reply to a message. It keeps track of the tracking ID and the component who is replaying
    pub fn reply<R>(&self, msg : R) -> Mailed<R> {
        Mailed::new(self.track_id, self.component_id, msg)
    }
}

impl<T> From<T> for Mailed<T> {
    fn from(value: T) -> Self {
        Self::new(0, 0, value)
    }
}

pub fn get_agent_address() -> (ActorAddr, ActorAddrRecv) {
    let id = next_actor_id();
    let (sender, receiver) = crossbeam_channel::bounded(256);
    (ActorAddr {
        id,
        sender
    },
    ActorAddrRecv {
        id,
        receiver
    })
}

pub struct ActorAddrRecv {
    receiver : Receiver<SiemMessage>,
    id : u64
}

pub struct ActorAddr {
    sender : Sender<SiemMessage>,
    id : u64
}

impl ActorAddr {
    pub fn new(sender : Sender<SiemMessage>) -> Self {
        let id = next_actor_id();
        Self {
            id,
            sender
        }
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn send<T : Into<SiemMessage>>(&self, msg : T) {
        let _ = self.sender.send(msg.into());
    }
}

impl ActorAddrRecv {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn recv(&self) -> &Receiver<SiemMessage> {
        &self.receiver
    }
}