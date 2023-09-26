mod fs;
mod security;

pub use fs::{File, FileSystem};
pub use security::{AttackInfo, ServerSecurity};

pub struct Server {
    name: String,
    fs: FileSystem,
    sec: ServerSecurity,
}

impl Server {
    pub fn new(name: String) -> Self {
        Server {
            name,
            fs: FileSystem::new(),
            sec: ServerSecurity::new(),
        }
    }
}
