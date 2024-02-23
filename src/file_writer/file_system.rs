use super::{FileWriter, Folder};
use std::{fs, io::Write, path::Path};

pub struct FileSystemWriter;

impl FileWriter for FileSystemWriter {
    fn write(&self, folders: Vec<Folder>) {
        for folder in folders {
            let path = Path::new(&folder.path);
            if !path.exists() {
                fs::create_dir_all(path).unwrap();
            }

            for file in folder.files {
                let file_path = path.join(&file.name);
                let mut fs_file = fs::File::create(file_path).unwrap();
                for line in file.conent {
                    fs_file.write_all(line.as_bytes()).unwrap();
                    fs_file.write_all(b"\n").unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_writer::File;
    use std::fs::File as FsFile;

    #[test]
    fn write_to_nested_folder_first_absolute_path() {
        // Arrange
        let folders: Vec<Folder> = vec![
            Folder {
                path: "/tmp/base".to_string(),
                files: vec![File {
                    name: "test1.txt".to_string(),
                    conent: vec![
                        "Absolute base folder line 1".to_string(),
                        "Absolute base folder line 2".to_string(),
                    ],
                }],
            },
            Folder {
                path: "/tmp/base/nested".to_string(),
                files: vec![File {
                    name: "test2.txt".to_string(),
                    conent: vec![
                        "Absolute Nested folder line 1".to_string(),
                        "Absolute Nested folder line 2".to_string(),
                    ],
                }],
            },
        ];

        // Act
        let writer = FileSystemWriter;
        writer.write(folders);

        // Assert
        let baseFile = FsFile::open("/tmp/base/test1.txt").unwrap();
        // assert

        // Clean-up
    }

    #[test]
    fn write_to_nested_folder_first_relative() {
        // Arrange
        let folders: Vec<Folder> = vec![
            Folder {
                path: "./test/base".to_string(),
                files: vec![File {
                    name: "test1.txt".to_string(),
                    conent: vec![
                        "Relative base folder line 1".to_string(),
                        "Relative base folder line 2".to_string(),
                    ],
                }],
            },
            Folder {
                path: "./test/base/nested".to_string(),
                files: vec![File {
                    name: "test2.txt".to_string(),
                    conent: vec![
                        "Relative nested folder line 1".to_string(),
                        "Relative nested folder line 2".to_string(),
                    ],
                }],
            },
        ];

        // Act
        let writer = FileSystemWriter;
        writer.write(folders);

        // Assert

        // Clean-up
    }
}
