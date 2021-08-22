use super::state::*;
use crate::auto_path::AutoPath;
use crate::manifest::Binder;
use derive_more::{Deref, DerefMut};
use rill_engine::tracers::tracer::Tracer;
use rill_protocol::flow::core::FlowMode;

#[derive(Debug, Deref, DerefMut, Clone)]
pub struct Switch {
    #[deref]
    #[deref_mut]
    tracer: Tracer<SwitchState>,
    _binder: Binder,
}

impl Switch {
    pub fn new(auto_path: impl Into<AutoPath>, spec: SwitchSpec) -> Self {
        let path = auto_path.into();
        let state = spec.into();
        let tracer = Tracer::new(state, path.into(), FlowMode::Realtime);
        let binder = Binder::new(&tracer);
        Self {
            tracer,
            _binder: binder,
        }
    }

    pub fn apply(&self, turn_on: bool) {
        let msg = SwitchEvent { turn_on };
        self.tracer.send(msg, None);
    }
}
