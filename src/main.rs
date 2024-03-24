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

use std::io::{self, stdout};

use color_eyre::{
    eyre::{bail, WrapErr},
    Result
};


mod tui;
mod errors;
mod frontend;
mod backend;

pub const APP_TITLE: &str = " File Explorer ";


fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;

    return app_result;
}


#[derive(Debug, Default)]
pub struct App {
    counter: isize,
    exit: bool
}
