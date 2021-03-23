//! The module with adapted and flexible tracers
//! that allow to use many types as parameters.

use anyhow::Error;
use rill_engine::tracers::data::{
    CounterTracer, DictTracer, GaugeTracer, HistogramTracer, LoggerTracer, PulseTracer, TableTracer,
};
use rill_engine::tracers::meta::EntryTracer;
pub use rill_protocol::io::provider::{Col, Row};
use std::ops::Deref;
use std::sync::Arc;

macro_rules! impl_tracer {
    ($wrapper:ident < $tracer:ident > ( $( $arg:ident : $typ:ty ),* )) => {

        /// Wrapper on tracer.
        #[derive(Debug, Clone)]
        pub struct $wrapper {
            tracer: Arc<$tracer>,
        }

        impl Deref for $wrapper {
            type Target = $tracer;

            fn deref(&self) -> &Self::Target {
                self.tracer.deref()
            }
        }

        impl $wrapper {
            /// Creates an instance of the tracer.
            pub fn create(path: impl AsRef<str>, $( $arg : $typ ),*) -> Result<Self, Error> {
                let path = path.as_ref().parse()?;
                let tracer = $tracer::new(path, $( $arg ),*);
                Ok(Self {
                    tracer: Arc::new(tracer),
                })
            }
        }
    };
}

impl_tracer!(Counter<CounterTracer>());

impl Counter {
    /// Increments value by the sepcific delta.
    pub fn inc(&self, delta: f64) {
        self.tracer.inc(delta, None);
    }
}

impl_tracer!(Gauge<GaugeTracer>(min: f64, max: f64));

impl Gauge {
    /// Increments value by the sepcific delta.
    pub fn set(&self, value: f64) {
        self.tracer.set(value, None);
    }
}

impl_tracer!(Histogram<HistogramTracer>(levels: Vec<f64>));

impl Histogram {
    /// Adds a value to the histogram.
    pub fn add(&self, value: f64) {
        self.tracer.add(value, None);
    }
}

impl_tracer!(Pulse<PulseTracer>(depth: Option<u32>));

impl Pulse {
    /// Increments the value by the specific delta.
    pub fn inc(&self, delta: f64) {
        self.tracer.inc(delta, None);
    }

    /// Decrements the value by the specific delta.
    pub fn dec(&self, delta: f64) {
        self.tracer.dec(delta, None);
    }

    /// Set the value.
    pub fn set(&self, delta: f64) {
        self.tracer.set(delta, None);
    }
}

impl_tracer!(Logger<LoggerTracer>());

impl Logger {
    /// Writes a message.
    pub fn log(&self, msg: impl ToString) {
        self.tracer.log(msg.to_string(), None);
    }
}

impl_tracer!(Dict<DictTracer>());

impl Dict {
    /// Assign a value to the key.
    pub fn set(&self, key: impl ToString, value: impl ToString) {
        self.tracer.set(key.to_string(), value.to_string(), None);
    }

    /// Remove value assigned to the key.
    pub fn del(&self, key: impl ToString) {
        self.tracer.del(key.to_string(), None);
    }
}

impl_tracer!(Table<TableTracer>(columns: Vec<(Col, impl ToString)>));

impl Table {
    // Reused, because timestamp is not required for them:
    // fn add_row
    // fn del_row

    /// Sets the cell of a table.
    pub fn set_cell(&self, row: Row, col: Col, value: impl ToString) {
        self.tracer.set_cell(row, col, value.to_string(), None);
    }
}

impl_tracer!(MetaEntry<EntryTracer>());
