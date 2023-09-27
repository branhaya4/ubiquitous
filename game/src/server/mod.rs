mod fs;
mod security;

use std::fmt::Display;

pub use fs::{File, FileSystem};
pub use security::{AttackInfo, ServerSecurity};

#[derive(Debug)]
pub struct Server {
    pub name: String,
    pub fs: FileSystem,
    pub sec: ServerSecurity,
}