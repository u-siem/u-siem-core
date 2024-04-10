use crossbeam_channel::SendError;

use crate::components::common::SiemMessage;

pub trait Sender: Send
{
    fn do_send(&self, msg: SiemMessage) -> Result<(), SendError<SiemMessage>>;

    fn try_send(&self, msg: SiemMessage) -> Result<(), SendError<SiemMessage>>;

    fn send(&self, msg: SiemMessage) -> Result<(), SendError<SiemMessage>>;

    fn boxed(&self) -> Box<dyn Sender + Sync>;

    fn hash(&self) -> usize;

    fn connected(&self) -> bool;
}

impl<S> Sender for Box<S>
where
    S: Sender + ?Sized
{
    fn do_send(&self, msg: SiemMessage) -> Result<(), SendError<SiemMessage>> {
        (**self).do_send(msg)
    }

    fn try_send(&self, msg: SiemMessage) -> Result<(), SendError<SiemMessage>> {
        (**self).try_send(msg)
    }

    fn send(&self, msg: SiemMessage) -> Result<(), SendError<SiemMessage>> {
        (**self).send(msg)
    }

    fn boxed(&self) -> Box<dyn Sender + Sync> {
        (**self).boxed()
    }

    fn hash(&self) -> usize {
        (**self).hash()
    }

    fn connected(&self) -> bool {
        (**self).connected()
    }
}
