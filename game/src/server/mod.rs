mod fs;
mod terminal;

use fs::FileSystem;
use terminal::Terminal;

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

    pub fn open_terminal() -> Terminal {
        todo!()
    }

    // ...
}