use std::collections::LinkedList;

pub struct FileSystem {
    files: LinkedList<File>
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            files: LinkedList::new()
        }
    }

    // ..
}

struct File {
    pub name: String,
    contents: String
}

impl File {
    fn new(name: String) -> Self {
        File {
            name,
            contents: String::new()
        }
    }

    fn get_contents(&self) -> &String {
        todo!()
    }

    fn set_contents(&mut self, contents: String) {
        todo!()
    }
}