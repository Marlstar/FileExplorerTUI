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
                self.handle_key_event(key);
            }
        }

        return Ok(());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        use KeyCode::*;
        match key_event.code {
            Char('q') => self.quit(),
            Down => self.navigateDown(),
            Up => self.navigateUp(),
            Enter => {let _ = self.itemInteract();},
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

    fn navigateUp(&mut self) {
        // todo: add wrapping?
        if self.selected > usize::MIN {
            self.selected -= 1;
        }
    }
    fn navigateDown(&mut self) {
        // todo: add wrapping?
        if self.selected <= match self.countItems() {
            Ok(a) => a-1,
            Err(_) => return
        }
        {
            self.selected += 1;
        }
    }

    fn itemInteract(&mut self) -> Result<(), bool> {
        let item = &self.currentItems[self.selected];
        if item.1 == FileType::Directory {
            match self.cd(item.0.clone()) {
                Ok(_) => {},
                Err(_) => return Err(false)
            };
        }

        return Ok(());
    }


    #[allow(unused_variables)]
    fn render_files(&mut self, area: Rect, buf: &mut Buffer) {
        // Styles
        let errorFiller = Line::styled("ERROR", Style::new().red());

        let folderStyle_selected = Style::new().cyan().on_gray();
        let folderStyle_unselected = Style::new().cyan();
        let fileStyle_selected = Style::new().light_green().on_gray();
        let fileStyle_unselected = Style::new().light_green();

        // Get raw items
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


        let parentFolderWidget = Line::styled("..",
            match self.selected {
                0 => folderStyle_selected,
                _ => folderStyle_unselected
            });

        // Vectors to store the items in
        let mut text_widgets: Vec<Line> = vec![parentFolderWidget]; // Starts with parent directory, others added in following loops
        self.currentItems = vec![(String::from(".."), FileType::Directory)]; // 

        let mut currentElement = 0;
        // Create widget and info for each item
        for dir in directories {
            currentElement += 1;
            let name: String = match dir.file_name() {
                Some(a) => match a.to_str() {
                    Some(a) => a.to_string(),
                    None => continue
                },
                None => continue
            };

            text_widgets.push(Line::styled(name.clone(), if self.selected == currentElement {folderStyle_selected} else {folderStyle_unselected}));
            self.currentItems.push((name, FileType::Directory));
        }
        for file in files {
            currentElement += 1;
            let name: String = match file.file_name() {
                Some(a) => match a.to_str() {
                    Some(a) => a.to_string(),
                    None => continue
                },
                None => continue
            };
            
            text_widgets.push(Line::styled(name.clone(), if self.selected == currentElement {fileStyle_selected} else {fileStyle_unselected}));
            self.currentItems.push((name, FileType::File));
        }

        Paragraph::new(text_widgets)
            .block(Block::new().borders(Borders::ALL))
            .render(area, buf);
    }

    fn render_file_info(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!("Status Panel\nSelected: {}\nTotal Items: {}\nCWD: {}", self.selected, match self.countItems() {Ok(a)=>a,Err(_)=>0}, self.cwd()))
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

        
        // Render the different parts of the application
        // Header
        self.render_header(header_area, buf);
        // Main application content
        self.render_files(file_area, buf);
        self.render_file_info(file_info_area, buf);
        // Footer
        self.render_footer(footer_area, buf);
    }
}