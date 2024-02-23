mod file_system;
pub use file_system::FileSystemWriter;

pub trait FileWriter {
    fn write(&self, folders: Vec<Folder>);
}

pub struct Folder {
    pub path: String,
    pub files: Vec<File>,
}

pub struct File {
    pub name: String,
    pub extension: String,
    pub content: Vec<String>,
}
