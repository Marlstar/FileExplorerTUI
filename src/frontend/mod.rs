use std::{ffi::OsStr, rc::Rc};

use crate::*;



impl App { 
    pub fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
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

impl App { // Render chunks
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

    #[allow(unused_variables)]
    fn render_files(&self, area: Rect, buf: &mut Buffer) {
        // Styles
        let errorFiller = Line::styled("ERROR", Style::new().red());

        let folderStyle_selected = Style::new().cyan().on_gray();
        let folderStyle_unselected = Style::new().cyan();
        let fileStyle_selected = Style::new().light_green().on_gray();
        let fileStyle_unselected = Style::new().light_green();

        // Code
        let files = match self.getType(FileType::File) {
            Ok(a) => a,
            Err(_) => {
                errorFiller.clone().render(area, buf);
                return;
            }
        };
        let directories = match self.getType(FileType::Directory) {
            Ok(a) => a,
            Err(_) => {
                errorFiller.clone().render(area, buf);
                return;
            }
        };

        let mut text_widgets: Vec<Line> = vec![];

        for dir in directories {
            let name: String = match dir.file_name() {
                Some(a) => match a.to_str() {
                    Some(a) => a.to_string(),
                    None => continue
                },
                None => continue
            };
            
            text_widgets.push(Line::styled(name, folderStyle_unselected));
        };
        for file in files {
            let name: String = match file.file_name() {
                Some(a) => match a.to_str() {
                    Some(a) => a.to_string(),
                    None => continue
                },
                None => continue
            };
            
            text_widgets.push(Line::styled(name, fileStyle_unselected));
        }

        Paragraph::new(text_widgets)
            .block(Block::new().borders(Borders::ALL))
            .render(area, buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::*;

        // Vertical chunks
        let vertical = Layout::vertical([
            Length(1), Min(0), Length(1) // Lengths are 1 line each and the Min(0) just makes a dynamic area in the middle
        ]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        // Main Chunks
        let horizontal_inner = Layout::horizontal([
            Min(40), Max(50)
        ]);
        let [file_area, file_info_area] = horizontal_inner.areas(inner_area);

        // File Info Area
        self.render_header(header_area, buf);

        self.render_files(file_area, buf);

        self.render_footer(footer_area, buf);
    }
}