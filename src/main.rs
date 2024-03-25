#![allow(non_snake_case, unused_imports, dead_code)]

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use std::io::{self, stdout, Write};
use std::fs::{self, File};

use color_eyre::{
    eyre::{bail, WrapErr},
    Result
};


mod tui;
mod errors;
mod frontend; use frontend::*;
mod backend; use backend::*;

pub const APP_TITLE: &str = " File Explorer ";
pub const DEFAULT_FOLDER: &str = "C:/Users/marle/Documents/CodingProjects/FileExplorerTUI/TUI_test_environment";

fn main() -> Result<()> {
    errors::install_hooks()?;
    println!("\n\n\n\n\n\n");

    let mut terminal = match tui::init() {
        Ok(a) => a,
        Err(_) => panic!("failed to hook terminal")
    };
    let mut app = match App::new() {
        Ok(a) => a,
        Err(_) => panic!("failed to create app")
    };
    let app_result: Result<()> = app.runFrontend(&mut terminal);
    return app_result;
}


pub struct App {
    // Frontend
    pub exit: bool,

    // Backend
    cwd: Vec<String>,
    home: String
}
impl App {
    pub fn new() -> Result<App, bool> {
        let a = App {
            

            // Frontend
            exit: false,

            // Backend
            cwd: match DirUtils::dirsFromPath(String::from(DEFAULT_FOLDER)) {
                Ok(a) => a,
                Err(_) => return Err(false)
            },
            home: String::from(DEFAULT_FOLDER)
        };
        Ok(a)
    }
}

// & //////////////////////////
// & ////////FRONTEND /////////
// & //////////////////////////
impl App {
    pub fn runFrontend(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        let app_result: Result<()>;
        while !self.exit {
            self.draw(terminal).wrap_err("failed to draw to terminal")?;
            self.handle_events().wrap_err("handling events failed")?;
        };
        app_result = Ok(());



        let _ = tui::restore();

        return app_result;
    }
}

// & ////////////////////////
// & ///////BACKEND /////////
// & ////////////////////////