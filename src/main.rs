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
pub const DEFAULT_FOLDER: &str = "C:/users/marle";

fn main() -> Result<()> {
    errors::install_hooks()?;
    let app = match App::new() {
        Ok(a) => a,
        Err(_) => panic!("failed to create app")
    };
    println!("\n\n\n\n <===== FILE EXPLORER =====>");
    println!("{:#?}", app.backend.listDir());

    return Ok(());
}
fn main_() -> Result<()> {
    errors::install_hooks()?;

    let mut app = match App::new() {
        Ok(a) => a,
        Err(_) => panic!("failed to create app")
    };
    let app_result: Result<()> = app.runFrontend();
    return app_result;
}


pub struct App {
    pub terminal: tui::Tui,
    frontend: AppFrontend,
    backend: AppBackend
}
impl App {
    pub fn new() -> Result<App, ()> {
        let a = App {
            terminal: match tui::init() {
                Ok(a) => a,
                Err(_) => return Err(())
            },
            frontend: AppFrontend::new(),
            backend: AppBackend::new()
        };
        Ok(a)
    }
}

impl App {
    pub fn runFrontend(&mut self) -> Result<()> {
        let app_result = self.frontend.run(&mut self.terminal);
        let _ = tui::restore();

        return app_result;
    }
}