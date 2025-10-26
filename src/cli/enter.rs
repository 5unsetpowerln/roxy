use std::path::PathBuf;

use clap::Parser;
use uuid::Uuid;

#[derive(Debug, Parser)]
pub(crate) struct Args {
    pub name: Option<String>,
    pub path: Option<PathBuf>,
    pub uuid: Option<Uuid>,
}
