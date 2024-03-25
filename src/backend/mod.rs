use std::path::{Path, PathBuf};

use crate::*;

pub enum FileType {
    Directory,
    File
}



impl App { // File commands
    pub fn cwd(&self) -> String {
        return DirUtils::pathFromDirs(&self.cwd);
    }

    pub fn listDir(&self) -> Result<Vec<PathBuf>, ()> {
        let mut files: Vec<PathBuf> = vec![];

        let paths = match fs::read_dir(self.cwd()) {
            Ok(a) => a,
            Err(_) => {println!("Failed to read dir {}", self.cwd()); return Err(())}
        };
        for path in paths {
            let p = path.expect("expected a valid path").path();
            files.push(p);
        }
        return Ok(files)
    }

    pub fn getType(&self, filetype: FileType) -> Result<Vec<PathBuf>, ()> {
        let files = self.listDir()?;
        let mut output: Vec<PathBuf> = vec![];
        for file in files {
            match filetype {
                FileType::File => {
                    if file.is_file() {
                        output.push(file);
                    }
                },
                FileType::Directory => {
                    if file.is_dir() {
                        output.push(file);
                    }
                }
            }
        }

        return Ok(output)
    }
}


pub enum CDError {
    InvalidPath
}

impl App { // Directory traversal
    pub fn cd(&mut self, folder: String) -> Result<(), CDError> {
        match folder.as_str() {
            ".." => {
                if self.cwd.len() <= 1 {return Err(CDError::InvalidPath)};
                self.cwd.pop();
                return Ok(());
            },
            _ => {
                let desPath = format!("{}/{}", self.cwd(), folder.as_str());
                if Path::new(desPath.as_str()).exists() {
                    self.cwd.push(folder);
                    return Ok(())
                }
                else {
                    return Err(CDError::InvalidPath)
                }
            }
        }
    }
}



pub struct DirUtils {}
impl DirUtils {
    pub fn dirsFromPath(path: String) -> Result<Vec<String>, ()> {
        let chunks_str: Vec<&str> = path.split('/').collect();
        let chunks: Vec<String> =  chunks_str.iter().map(|&x| String::from(x)).collect();
        return Ok(chunks);
    }
    pub fn pathFromDirs(dirs: &Vec<String>) -> String {
        let mut s = format!("{}", dirs[0]);
        for element in &dirs[1..dirs.len()] {
            s = format!("{}{}", s, format!("/{}", element))
        }
        return s;
    }
}