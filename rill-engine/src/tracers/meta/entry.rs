use crate::tracers::tracer::Tracer;
use derive_more::{Deref, DerefMut};
use rill_protocol::flow::meta::entry::{EntryEvent, EntryFlow, EntryState};
use rill_protocol::io::provider::{EntryId, Path};

/// This tracer that informs about entries.
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct EntryTracer {
    tracer: Tracer<EntryFlow>,
}

impl EntryTracer {
    /// Create a new instance of the `Tracer`.
    pub fn new(path: Path) -> Self {
        let flow = EntryFlow;
        let state = EntryState::new();
        let tracer = Tracer::new_tracer(flow, state, path, None);
        Self { tracer }
    }

    /// Add an entry
    pub fn add(&self, name: EntryId) {
        let data = EntryEvent::AddEntry { name };
        self.tracer.send(data, None);
    }

    /// Remove an entry
    pub fn del(&self, name: EntryId) {
        let data = EntryEvent::RemoveEntry { name };
        self.tracer.send(data, None);
    }
}
