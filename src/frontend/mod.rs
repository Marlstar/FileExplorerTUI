use crate::*;

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handling events failed")?;
        }
        return Ok(());
    }

    pub fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
            _ => {}
        };

        return Ok(());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.change_counter(1),
            KeyCode::Down => self.change_counter(-1),
            _ => {}
        }
    }

    fn change_counter(&mut self, num: isize) {
        self.counter += num;
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}


impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(APP_TITLE.bold());

        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Down>".blue().bold(),
            " Increment ".into(),
            "<Up>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

            let counter_text = Text::from(vec![Line::from(vec![
                "Value: ".into(),
                self.counter.to_string().yellow(),
            ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
    
}