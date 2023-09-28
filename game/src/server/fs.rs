#[derive(Debug, Default)]
pub struct FileSystem {
    files: Vec<File>,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    contents: String,
}

impl File {
    fn new(name: String) -> Self {
        File {
            name,
            contents: String::new(),
        }
    }

    fn get_contents(&self) -> &String {
        &self.contents // Return self contents
    }

    fn set_contents(&mut self, contents: String) {
        self.contents = contents;
    }
}
