mod dev;
mod fs;
mod security;

pub use dev::Device;
pub use fs::{File, FileSystem};
pub use security::{AttackInfo, ServerSecurity, SecurityState};
use std::fmt::Display;


#[derive(Debug)]
pub struct Server {
    pub name: String,
    pub fs: FileSystem,
    pub sec: ServerSecurity,
    pub devices: Vec<Device>,
}
