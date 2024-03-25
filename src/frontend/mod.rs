use std::rc::Rc;

use crate::*;


#[derive(Debug)]
pub struct AppFrontend {
    exit: bool
}
impl AppFrontend {
    pub fn new() -> AppFrontend {
        AppFrontend {
            exit: false
        }
    }
}

impl AppFrontend { 
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            self.draw(terminal).wrap_err("failed to draw to terminal")?;
            self.handle_events().wrap_err("handling events failed")?;
        }

        return Ok(());
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;

        return Ok(());
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press { // Falling edge detector
                use KeyCode::*;

                match key.code { // * Keypresses go here
                    _ => {}
                }
            }
        }

        return Ok(());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.exit = true;
    }
}

impl AppFrontend { // Render chunks
    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        Line::raw(" File Explorer ")
            .centered()
            .render(area, buf)
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Line::from(vec![
            Span::raw(" Navigation "),
            Span::styled("<Up/Down>", Style::new().light_blue()),
            Span::raw(" | Interact "),
            Span::styled("<Enter>", Style::new().light_blue()),
            Span::raw(" | Quit "),
            Span::styled("<Q> ", Style::new().light_blue())
        ])
            .centered()
            .render(area, buf);
    }

    fn render_files
}

impl Widget for &AppFrontend {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::*;

        // Vertical chunks
        let vertical = Layout::vertical([
            Length(1), Min(0), Length(1) // Lengths are 1 line each and the Min(0) just makes a dynamic area in the middle
        ]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        // Main Chunks
        let horizontal_inner = Layout::horizontal([
            Min(40), Max(20)
        ]);
        let [file_area, file_info_area] = horizontal_inner.areas(inner_area);

        self.render_header(header_area, buf);

        self.render_footer(footer_area, buf);
    }
}