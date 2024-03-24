use std::path::PathBuf;

use crate::*;

pub enum FileType {
    Directory,
    File
}

pub struct AppBackend {
    cwd: Vec<String>,
    home: String
}
impl AppBackend {
    pub fn new() -> AppBackend {
        AppBackend {
            cwd: vec![String::from(DEFAULT_FOLDER)],
            home: String::from(DEFAULT_FOLDER)
        }
    }
}


impl AppBackend { // File commands
    pub fn path(&self) -> String {
        return DirUtils::pathFromDirs(&self.cwd);
    }

    pub fn listDir(&self) -> Result<Vec<PathBuf>, ()> {
        let mut files: Vec<PathBuf> = vec![];

        let paths = match fs::read_dir(self.path()) {
            Ok(a) => a,
            Err(_) => {println!("Failed to read dir {}", self.path()); return Err(())}
        };
        for path in paths {
            let p = path.expect("expected a valid path").path();
            files.push(p);
        }
        return Ok(files)
    }
}


pub struct DirUtils {}
impl DirUtils {
    pub fn dirsFromPath(path: String) -> Result<Vec<String>, ()> {
        let chunks_str: Vec<&str> = path.split('/').collect();
        let mut chunks: Vec<String> =  chunks_str.iter().map(|&x| String::from(x)).collect();
        let drive: String = {
            let c = chunks[0].as_str();

            String::from(match c.chars().nth(0) {Some(a) => a, None => return Err(())})
        };
        chunks[0] = drive;
        return Ok(chunks);
    }
    pub fn pathFromDirs(dirs: &Vec<String>) -> String {
        let mut s = format!("{}:", dirs[0]);
        for element in &dirs[1..dirs.len()] {
            s = format!("{}{}", s, format!("/{}", element))
        }
        return s[..s.len()-1].to_string();
    }
}