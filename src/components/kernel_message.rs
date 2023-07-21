use std::time::Duration;

use chrono::Utc;
use crossbeam_channel::Sender;

use crate::prelude::{types::LogString, Notification, NotificationLevel, SiemMessage, SiemResult};

#[derive(Clone)]
pub struct KernelMessager {
    component_id: u64,
    component_name: String,
    channel: Sender<SiemMessage>,
    level: NotificationLevel,
}

impl Default for KernelMessager {
    fn default() -> Self {
        let (channel, _) = crossbeam_channel::bounded(1);
        Self {
            component_id: Default::default(),
            component_name: Default::default(),
            channel,
            level: NotificationLevel::Info,
        }
    }
}

impl KernelMessager {
    pub fn new(id: u64, name: String, channel: Sender<SiemMessage>) -> Self {
        Self {
            component_id: id,
            component_name: name,
            channel,
            level: NotificationLevel::Info,
        }
    }
    pub fn set_level(&mut self, level: NotificationLevel) {
        self.level = level;
    }

    pub fn send<T: Into<SiemMessage>>(&self, msg: T) -> SiemResult<()> {
        let msg: SiemMessage = msg.into();
        Ok(self.channel.send(msg)?)
    }

    pub fn try_send<T: Into<SiemMessage>>(&self, msg: T) -> SiemResult<()> {
        let msg: SiemMessage = msg.into();
        Ok(self.channel.try_send(msg)?)
    }

    pub fn send_timeout<T: Into<SiemMessage>>(&self, msg: T, timeout: Duration) -> SiemResult<()> {
        let msg: SiemMessage = msg.into();
        Ok(self.channel.send_timeout(msg, timeout)?)
    }

    fn _send(&self, log: String, level: NotificationLevel) {
        if self.level < level {
            return;
        }
        let _ = self
            .channel
            .try_send(SiemMessage::Notification(Notification {
                component: self.component_id,
                timestamp: Utc::now().timestamp_millis(),
                component_name: LogString::Owned(self.component_name.clone()),
                log: LogString::Owned(log),
                level: level,
            }));
    }
    pub fn log(&self, log: String, level: NotificationLevel) {
        self._send(log, level);
    }
    pub fn trace(&self, log: String) {
        self._send(log, NotificationLevel::Trace);
    }
    pub fn debug(&self, log: String) {
        self._send(log, NotificationLevel::Debug);
    }
    pub fn info(&self, log: String) {
        self._send(log, NotificationLevel::Info);
    }
    pub fn warn(&self, log: String) {
        self._send(log, NotificationLevel::Warn);
    }
    pub fn error(&self, log: String) {
        self._send(log, NotificationLevel::Error);
    }
}
