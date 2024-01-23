use std::time::{Duration, Instant};

use crossbeam_channel::{Sender, Receiver, Select};

use crate::prelude::{SiemResult, SiemLog};

use super::{command::{SiemCommandHeader, SiemCommandCall, SiemCommandResponse}, dataset::{SiemDataset, holder::DatasetHolder}, task::{SiemTask, SiemTaskResult}, common::{SiemMessage, SiemComponentCapabilities, Notification}, storage::SiemComponentStateStorage, SiemComponent, alert::SiemAlert};

/// A easy to customize Siem Component that does not require complex logic inside the run() method.
/// 
/// It uses the trait SimplifiedComponent for the logic
/// 
///```
///use usiem::prelude::*;
///#[derive(Clone)]
///struct BasicComponent {}
///impl BasicComponent {
///    pub fn new() -> Self {
///        Self {}
///    }
///}
///impl SimplifiedComponent for BasicComponent {
///    fn on_log(&mut self, mut log : SiemLog) -> SiemResult<Option<SiemLog>> {
///        log.add_field("PROCESSED", "PROCESSED".into());
///        Ok(Some(log))
///    }
///
///    fn capabilities(&self) -> SiemComponentCapabilities {
///        SiemComponentCapabilities::new(
///            LogString::Borrowed("Basic"),
///            LogString::Borrowed("Basic component"),
///            LogString::Borrowed(""),
///            vec![],
///            vec![],
///            vec![],
///            vec![],
///        )
///    }
///
///    fn duplicate(&self) -> Box<dyn SimplifiedComponent> {
///        Box::new(self.clone())
///    }
///}
/// ```
#[allow(unused_variables)]
pub trait SimplifiedComponent : Send {
    /// Allow to store information about this component like the state or configurations.
    fn set_storage(&mut self, conn: Box<dyn SiemComponentStateStorage>) {}

    /// Capabilities and actions that can be performed by this component
    fn capabilities(&self) -> SiemComponentCapabilities;

    /// Allows the Kernel to duplicate this component
    fn duplicate(&self) -> Box<dyn SimplifiedComponent>;

    /// Initialize the component with the datasets before executing run
    fn set_datasets(&mut self, datasets: DatasetHolder) {}

    /// Executed when the component receives a command to execute
    fn on_command(&mut self, header : SiemCommandHeader, action : SiemCommandCall) -> SiemResult<()> {
        Ok(())
    }
    fn on_response(&mut self, header : SiemCommandHeader, action : SiemCommandResponse) -> SiemResult<()> {
        Ok(())
    }
    /// Executed when the component receives a log. Return Ok(None) to filter and remove the log
    fn on_log(&mut self, log : SiemLog) -> SiemResult<Option<SiemLog>> {
        Ok(Some(log))
    }
    /// Called when the component updates i
    fn on_dataset(&mut self, dataset : SiemDataset) -> SiemResult<()> {
        Ok(())
    }
    fn on_alert(&mut self, alert : SiemAlert) -> SiemResult<()> {
        Ok(())
    }
    /// The component received a task to execute
    fn on_task(&mut self, header : SiemCommandHeader, task : SiemTask)-> SiemResult<()> {
        Ok(())
    }
    fn on_task_result(&mut self, header : SiemCommandHeader, result : SiemTaskResult)-> SiemResult<()> {
        Ok(())
    }
    /// Errors are roported as debug
    fn on_notification(&mut self, notifiation : Notification)-> SiemResult<()> {
        Ok(())
    }

    /// Perform actions. Return the number of millisecons to call the tick function again, or None for not (Default). If ab error is returned the component will end its execution.
    fn tick(&mut self) -> SiemResult<Option<u64>> {
        Ok(None)
    }
}

/// Encapsulation of Simplified components
pub struct SimpleComponent {
    component : Box<dyn SimplifiedComponent>,
    channel : (Sender<SiemMessage>, Receiver<SiemMessage>),
    logs : (Sender<SiemLog>, Receiver<SiemLog>)
}

impl SimpleComponent {
    pub fn new(component : Box<dyn SimplifiedComponent>) -> Self {
        let channel = crossbeam_channel::bounded(1024);
        let logs = crossbeam_channel::bounded(1);
        Self {
            component,
            channel,
            logs,
        }
    }
}

impl SiemComponent for SimpleComponent {
    fn local_channel(&self) -> Sender<SiemMessage> {
        self.channel.0.clone()
    }

    fn set_log_channel(&mut self, sender: Sender<SiemLog>, receiver: Receiver<SiemLog>) {
        self.logs = (sender, receiver);
    }

    fn run(&mut self) -> SiemResult<()> {
        let mut select = Select::new();
        select.recv(&self.channel.1);
        select.recv(&self.logs.1);
        let mut component = self.component.duplicate();
        let mut deadline = deadline_of_tick(component.tick()?);
        loop {
            let oper = if let Some(t) =  deadline {
                match select.select_deadline(t) {
                    Ok(v) => v,
                    Err(_) => {
                        deadline = deadline_of_tick(component.tick()?);
                        continue
                    }
                }
            }else {
                select.select()
            };
            let result = match oper.index() {
                0 => {
                    let msg = match oper.recv(&self.channel.1) {
                        Ok(v) => v,
                        Err(_) => break
                    };
                    process_message(msg, &mut component, &self)
                },
                1 => {
                    let msg = match oper.recv(&self.logs.1) {
                        Ok(v) => v,
                        Err(_) => break
                    };
                    process_log_for_component(msg, &mut component, &self)
                },
                _ => None
            };
            match result {
                Some(Err(err)) => {
                    crate::debug!("Error: {:?}", err);
                },
                None => break,
                _ => {}
            }
        }
        Ok(())
    }

    fn set_storage(&mut self, conn: Box<dyn SiemComponentStateStorage>) {
        self.component.set_storage(conn);
    }

    fn capabilities(&self) -> SiemComponentCapabilities {
        self.component.capabilities()
    }

    fn duplicate(&self) -> Box<dyn SiemComponent> {
        Box::new(SimpleComponent::new(self.component.duplicate()))
    }

    fn set_datasets(&mut self, datasets: DatasetHolder) {
        self.component.set_datasets(datasets);
    }
}

fn deadline_of_tick(tick : Option<u64>) -> Option<Instant> {
    match tick {
        Some(v) => Some(Instant::now() + Duration::from_millis(v)),
        None => None,
    }
}

#[allow(unreachable_patterns)]
fn process_message( msg : SiemMessage, component : &mut Box<dyn SimplifiedComponent>, state : &SimpleComponent) -> Option<SiemResult<()>> {
    Some(match msg {
        SiemMessage::Command(header, action) => {
            if let SiemCommandCall::STOP_COMPONENT(_) = &action {
                return None
            }
            component.on_command(header, action)
        },
        SiemMessage::Response(header, response) => component.on_response(header, response),
        SiemMessage::Log(log) => return process_log_for_component(log, component, state),
        SiemMessage::Notification(v) => component.on_notification(v),
        SiemMessage::Dataset(v) => component.on_dataset(v),
        SiemMessage::Alert(v) => component.on_alert(v),
        SiemMessage::Task(header, task) => component.on_task(header, task),
        SiemMessage::TaskResult(header, result) => component.on_task_result(header, result),
        _ => Ok(()),
    })
}
fn process_log_for_component(log : SiemLog, component : &mut Box<dyn SimplifiedComponent>, state : &SimpleComponent)-> Option<SiemResult<()>> {
    let log = match component.on_log(log) {
        Ok(v) => v,
        Err(e) => return Some(Err(e))
    };
    if let Some(log) = log {
        if let Err(err) = state.logs.0.send(log) {
            // Cannot send log to queue -> print to console 
            crate::info!("{}", serde_json::to_string(&err.0).unwrap_or_default());
            return None
        }
    }
    Some(Ok(()))
}


#[cfg(test)]
mod tst {
    use super::*;
    use std::time::Duration;

    use crate::prelude::{SiemLog, SiemResult, SiemMessage, SiemCommandCall, SiemCommandHeader, types::LogString};

    use super::SimplifiedComponent;

    #[derive(Clone)]
    struct BasicComponent {}
    impl BasicComponent {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl SimplifiedComponent for BasicComponent {
        fn on_log(&mut self, mut log : SiemLog) -> SiemResult<Option<SiemLog>> {
            log.add_field("PROCESSED", "PROCESSED".into());
            Ok(Some(log))
        }

        fn capabilities(&self) -> SiemComponentCapabilities {
            SiemComponentCapabilities::new(
                LogString::Borrowed("SimpleComponent"),
                LogString::Borrowed("SimpleComponent"),
                LogString::Borrowed(""),
                vec![],
                vec![],
                vec![],
                vec![],
            )
        }

        fn duplicate(&self) -> Box<dyn SimplifiedComponent> {
            Box::new(self.clone())
        }
    }
    #[test]
    fn should_execute_simplified_component() {
        let (comp_sender, log_r) = crossbeam_channel::bounded(1024);
        let (log_s, comp_recv) = crossbeam_channel::bounded(1024);
        let mut component = SimpleComponent::new(Box::new(BasicComponent::new()));
        component.set_log_channel(comp_sender, comp_recv);
        let k_sender = component.local_channel();
        std::thread::spawn(move || {
            component.run()
        });
        log_s.send(SiemLog::new("message", 123, "origin")).unwrap();
        let log = log_r.recv_timeout(Duration::from_millis(1000)).unwrap();
        assert_eq!("message", log.message());
        assert!(log.has_field("PROCESSED"));
        std::thread::sleep(Duration::from_millis(1000));
        k_sender.send(SiemMessage::Command(SiemCommandHeader::default(), SiemCommandCall::STOP_COMPONENT("".into()))).unwrap();
    }
}