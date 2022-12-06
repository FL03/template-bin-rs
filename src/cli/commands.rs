/*
    Appellation: commands <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Subcommand;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Connect {
        #[clap(long, short, value_parser)]
        address: String,
    },
    System {
        #[arg(action = clap::ArgAction::SetTrue, long)]
        up: bool,
    },
}

impl Commands {
    pub async fn handler(&self) -> &Self {
        tracing::info!("Processing commands issued to the cli...");

        self
    }
}
