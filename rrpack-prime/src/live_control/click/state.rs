use rill_protocol::flow::core::Flow;
use rill_protocol::io::provider::StreamType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickState {
    pub label: String,
}

impl ClickState {
    pub fn new(label: String) -> Self {
        Self { label }
    }
}

impl Flow for ClickState {
    type Action = ClickAction;
    type Event = ClickEvent;

    fn stream_type() -> StreamType {
        StreamType::from(module_path!())
    }

    fn apply(&mut self, _event: Self::Event) {}
}

pub type ClickAction = ();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickEvent;
