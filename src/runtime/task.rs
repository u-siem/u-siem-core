use crossbeam_channel::{Select, Sender};

use crate::{components::{collector::{LogCollectorContext, LogCollectorHandler}, common::SiemMessage, parser::{LogParsingContext, LogProcessorHandler}}, error};

use super::{actor::{Actor, ActorContext}, address::{get_agent_address, ActorAddr}};

pub fn run_parser<A>(mut actor : A, mut ctx : LogParsingContext)  -> ActorAddr
where 
    A: LogProcessorHandler
{
    actor.init(&mut ctx);
    let (sender, receiver) = get_agent_address();

    std::thread::spawn(move || {
        let mut select = Select::new();
        let slf_rec = select.recv(&receiver.recv());
        let log_receiver = ctx.receiver().clone();
        let _ctx_rec = select.recv(&log_receiver);
        loop {
            let id = match select.try_ready() {
                Ok(v) => v,
                Err(_) => {
                    if let Err(e) = actor.step(&mut ctx) {
                        error!("Error executing component: {:?}", e);
                        break;
                    }
                    continue
                },
            };
            if id == slf_rec {
                if let Ok(msg)= receiver.recv().try_recv() {
                    match msg {
                        SiemMessage::Command(cmd) => actor.command(cmd.mask(&receiver), &mut ctx),
                        SiemMessage::Response(rsp) => actor.response(rsp.mask(&receiver), &mut ctx),
                        SiemMessage::Log(log) => actor.parse_log(log, &mut ctx),
                        SiemMessage::Dataset(d) => {
                            let typ = d.dataset_type();
                            ctx.update_dataset(d);
                            actor.updated_dataset(typ);
                        },
                        SiemMessage::Alert(_) => todo!(),
                        SiemMessage::Task(tsk) => actor.task(tsk.mask(&receiver), &mut ctx),
                        SiemMessage::TaskResult(tsk) => actor.task_result(tsk.mask(&receiver), &mut ctx),
                        _ => {}
                    }
                }   
            }else {
                if let Ok(log) = ctx.receiver().recv() {
                    actor.parse_log(log, &mut ctx);
                }
            }
        }
    });
    sender
}

pub fn run_collector<A>(mut actor : A, mut ctx : LogCollectorContext) -> ActorAddr
where 
    A: LogCollectorHandler
{
    actor.init(&mut ctx);
    let (sender, receiver) = get_agent_address();
    std::thread::spawn(move || {
        loop {
            match receiver.recv().try_recv() {
                Ok(v) => match v {
                    SiemMessage::Command(cmd) => actor.command(cmd.mask(&receiver), &mut ctx),
                    SiemMessage::Response(rsp) => actor.response(rsp.mask(&receiver), &mut ctx),
                    SiemMessage::Dataset(d) => {
                        let typ = d.dataset_type();
                        ctx.update_dataset(d);
                        actor.updated_dataset(typ);
                    },
                    SiemMessage::Alert(_) => todo!(),
                    SiemMessage::Task(tsk) => actor.task(tsk.mask(&receiver), &mut ctx),
                    SiemMessage::TaskResult(tsk) => actor.task_result(tsk.mask(&receiver), &mut ctx),
                    SiemMessage::Log(log) => ctx.ingest(log),
                    _ => {}
                },
                Err(e) => {
                    if e.is_disconnected() {
                        break
                    }
                },
            };
            if let Err(e) = actor.step(&mut ctx) {
                error!("Error executing component: {:?}", e);
                break;
            } 
        }
    });
    sender
}

pub fn run_generic<A>(mut actor : A, mut context : A::Context) -> Sender<SiemMessage>
where 
    A: Actor 
{
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
            if let Err(e) = actor.step(&mut context) {
                error!("Error executing component: {:?}", e);
                break;
            } 
        }
    });
    sender
}

fn notify_change_to_actor<A>(msg : SiemMessage, actor : &mut A, ctx : &mut A::Context) 
where 
    A: Actor 
{
    match msg {
        SiemMessage::Command(cmd) => actor.command(cmd, ctx),
        SiemMessage::Response(rsp) => actor.response(rsp, ctx),
        SiemMessage::Log(_) => todo!(),
        SiemMessage::Notification(_) => todo!(),
        SiemMessage::Dataset(_) => todo!(),
        SiemMessage::Alert(_) => todo!(),
        SiemMessage::Task(_) => todo!(),
        SiemMessage::TaskResult(_) => todo!(),
    }
}