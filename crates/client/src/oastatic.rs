//! Static Information for the client
//!
//! This information is usually compiled into the executable at compile time
//! or Lazily executed at runtime
#![allow(dead_code)]

// Game Client Meta Information
pub const GAME_CLIENT_NAME: &str = "Open Arena Fortress";
pub const GAME_CLIENT_NAME_INTERNAL: &str = env!("CARGO_PKG_NAME");
pub const GAME_CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GAME_CLIENT_VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub const GAME_CLIENT_VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
pub const GAME_CLIENT_VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
pub const GAME_CLIENT_VERSION_PRE: &str = env!("CARGO_PKG_VERSION_PRE");
