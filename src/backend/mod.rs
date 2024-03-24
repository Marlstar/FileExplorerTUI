use crate::*;

pub struct AppBackend {
    cwd: String
}

impl AppBackend { // File commands

}

impl AppBackend {
    pub fn new() -> AppBackend {
        AppBackend {
            cwd: String::new()
        }
    }
}