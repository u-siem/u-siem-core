use crate::{components::{collector::{LogCollectorAddr, LogCollectorContext, LogCollectorHandler}, common::SiemMessage, parser::{LogParsingContext, LogProcessorHandler}}, error, prelude::store::DatasetStore};

use super::actor::{Actor, ActorContext};

pub fn run_parser<A>(mut actor : A, datasets : DatasetStore) 
where 
    A: LogProcessorHandler
{
    let mut context = LogParsingContext::new();
    context.set_datasets(datasets);
    actor.init(&mut context);
    std::thread::spawn(move || {
        actor.work()
    });
}

pub fn run_collector<A>(mut actor : A, datasets : DatasetStore) -> LogCollectorAddr
where 
    A: LogCollectorHandler
{
    let mut context = LogCollectorContext::new();
    context.set_datasets(datasets);
    actor.init(&mut context);
    let (sender, receiver) = crossbeam_channel::bounded(256);
    std::thread::spawn(move || {
        loop {
            match receiver.try_recv() {
                Ok(v) => notify_change_to_actor(v, &mut actor, &mut context),
                Err(e) => {
                    if e.is_disconnected() {
                        break
                    }
                },
            };
            if let Err(e) = actor.work() {
                error!("Error executing component: {:?}", e);
                break;
            } 
        }
        
    });
    LogCollectorAddr { sender }
}

fn notify_change_to_actor<A>(msg : SiemMessage, actor : &mut A, ctx : &mut A::Context) 
where 
    A: Actor 
{
    match msg {
        SiemMessage::Command(_, cmd) => actor.command(cmd, ctx),
        SiemMessage::Response(_, rsp) => actor.response(rsp, ctx),
        SiemMessage::Log(_) => todo!(),
        SiemMessage::Notification(_) => todo!(),
        SiemMessage::Dataset(_) => todo!(),
        SiemMessage::Alert(_) => todo!(),
        SiemMessage::Task(_, _) => todo!(),
        SiemMessage::TaskResult(_, _) => todo!(),
    }
}