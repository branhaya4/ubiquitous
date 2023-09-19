pub struct FileSystem {
    files: Vec<File>
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            files: Vec::new()
        }
    }
}

pub struct File {
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
        &self.contents // Return self contents
    }

    fn set_contents(&mut self, contents: String) {
        self.contents = contents;
    }
}