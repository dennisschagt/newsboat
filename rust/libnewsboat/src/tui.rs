use crate::ui;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use std::error;
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { terminal }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn error::Error>> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), EnterAlternateScreen)?;
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), Box<dyn error::Error>> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn draw(&mut self) -> Result<(), Box<dyn error::Error>> {
        self.terminal.draw(|frame| ui::render(frame))?;
        Ok(())
    }
}

pub struct TuiType {
    tui: Tui<CrosstermBackend<std::io::Stderr>>,
}

impl TuiType {
    pub fn new() -> Self {
        let backend = CrosstermBackend::new(std::io::stderr());
        let terminal = Terminal::new(backend).expect("failed to create terminal");
        let tui = Tui::new(terminal);
        TuiType { tui }
    }

    pub fn init(&mut self) {
        self.tui.init().expect("failed to initalize terminal");
    }

    pub fn exit(&mut self) {
        self.tui.exit().expect("failed to exit terminal");
    }

    pub fn draw(&mut self) {
        self.tui.draw().expect("failed to draw terminal");
    }

    pub fn wait_for_event(&self) -> String {
        loop {
            let event = event::read().expect("failed to read event");
            match event {
                Event::Resize(_, _) => break "RESIZE".to_owned(),
                Event::Key(key) => break key_to_newsboat_input_event(key),
                Event::FocusGained => (),
                Event::FocusLost => (),
                Event::Mouse(_) => (),
                Event::Paste(_) => (),
            };
        }
    }
}

fn key_to_newsboat_input_event(key: KeyEvent) -> String {
    match key.code {
        KeyCode::Backspace => "BACKSPACE".to_owned(),
        KeyCode::Enter => "ENTER".to_owned(),
        KeyCode::Left => "LEFT".to_owned(),
        KeyCode::Right => "RIGHT".to_owned(),
        KeyCode::Up => "UP".to_owned(),
        KeyCode::Down => "DOWN".to_owned(),
        KeyCode::PageUp => "PPAGE".to_owned(),
        KeyCode::PageDown => "NPAGE".to_owned(),
        KeyCode::Home => "HOME".to_owned(),
        KeyCode::End => "END".to_owned(),
        KeyCode::Esc => "ESC".to_owned(),
        KeyCode::Tab => "TAB".to_owned(),
        KeyCode::F(n) => format!("F{}", n),
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) && c.is_alphabetic() {
                format!("^{}", c.to_uppercase())
            } else {
                format!("{}", c)
            }
        }
        //BackTab,
        //Delete,
        //Insert,
        code => format!("Unknown: {:?}", code),
    }
}
