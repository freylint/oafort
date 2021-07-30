//! Client Command Line Interface Module

use structopt::StructOpt;

/// Client CLI Args Struct
///
/// Gets parsed at runtime initialization.
/// is used for global application configuration
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct ClientConfig {
    /// Sound disable toggle
    #[structopt(long)]
    pub nosnd: bool,
    /// Diagnostics enabling toggle
    #[structopt(long)]
    pub diag: bool,
}
