use super::Recorder;
use crate::tracers::tracer::ControlReceiver;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Context, IdOf, LiteTask, TaskEliminated, TaskError};
use rill_protocol::flow::core;
use tokio::sync::mpsc;

impl<T: core::Flow> Recorder<T> {
    pub(super) fn spawn_callback_worker(&mut self, ctx: &mut Context<Self>) {
        let (tx, rx) = mpsc::unbounded_channel();
        self.callback = Some(tx);
        let worker = CallbackWorker { receiver: rx };
        ctx.spawn_task(worker, (), ());
    }
}

pub struct CallbackWorker<T: core::Flow> {
    receiver: ControlReceiver<T>,
}

#[async_trait]
impl<T: core::Flow> LiteTask for CallbackWorker<T> {
    type Output = ();

    async fn interruptable_routine(mut self) -> Result<Self::Output, Error> {
        while let Some(envelope) = self.receiver.recv().await {}
        Ok(())
    }
}

#[async_trait]
impl<T: core::Flow> TaskEliminated<CallbackWorker<T>, ()> for Recorder<T> {
    async fn handle(
        &mut self,
        _id: IdOf<CallbackWorker<T>>,
        _tag: (),
        _result: Result<(), TaskError>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        // TODO: Drop unfinished tasks
        Ok(())
    }
}
