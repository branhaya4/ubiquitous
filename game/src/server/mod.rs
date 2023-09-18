mod fs;
mod terminal;

pub use fs::FileSystem;
pub use terminal::Terminal;

pub struct Server {
    name: String,
    fs: FileSystem
    // ...
}

impl Server {
    pub fn new(name: String) -> Self {
        Server {
            name,
            fs: FileSystem::new()
        }
    }
}