// --------------------------------------------------------------------
// Gufo Agent: Common definitions
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

pub(crate) mod collectable;
pub(crate) mod error;
pub(crate) mod label;
pub mod metrics;
pub(crate) mod timing;
pub use collectable::{Collectable, Measure, Value};
pub use error::AgentError;
pub use label::{Label, Labels, LabelsConfig};
pub use timing::Timing;
