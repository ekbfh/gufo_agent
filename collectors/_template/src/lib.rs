// --------------------------------------------------------------------
// Gufo Agent: _template collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{counter, gauge, AgentError, Collectable, Measure};
use serde::Deserialize;

// Collector config
#[derive(Deserialize)]
pub struct Config {
    // !!! Define configuration fields
}

// Collector structure
pub struct Collector {
    // !!! Define internal fields
}

// Generated metrics
// !!! Define counter! and gauge!
gauge!(mymetric, "Total DNS requests performed");

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        Ok(Self {
            // !!! Fill collector fields from config
        })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "_template";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Collect data
        // Push result
        Ok(vec![
            mymetric(0),
        ])
    }
}
