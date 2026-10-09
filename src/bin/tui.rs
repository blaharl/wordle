use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
};

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};

use std::io::prelude::*;
use std::net::TcpStream;

use wordle::{
    msg::{ClientMsg, ServerMsg},
    words::{Color::*, GuessState},
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> Result<()> {
    App::new().run(terminal)
}

struct App {
    input: String,
    responses: Vec<ServerMsg>,
    guesses: Vec<String>,
    stream: TcpStream,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            responses: Vec::new(),
            guesses: Vec::new(),
            stream: TcpStream::connect("127.0.0.1:4321").unwrap(),
        }
    }

    fn enter_char(&mut self, new_char: char) {
        if self.input.len() > 50 {
            return;
        }
        let new_char = new_char.to_ascii_uppercase();
        self.input.push(new_char);
    }

    fn delete_char(&mut self) {
        if !self.input.is_empty() {
            self.input.pop();
        }
    }

    fn receive_response(&mut self) -> Result<()> {
        let mut buffer = vec![0; 1024];
        let n = self.stream.read(&mut buffer)?;
        let server_msg = ServerMsg::from_slice(&buffer[..n]);
        self.responses.push(server_msg);
        Ok(())
    }

    fn submit_message(&mut self) -> Result<()> {
        let guess = self.input.trim().to_string();
        self.guesses.push(guess.clone());
        let client_msg = ClientMsg::new(guess);
        self.stream.write_all(client_msg.to_string().as_bytes())?;
        self.input.clear();
        Ok(())
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(key) = event::read()?.as_key_press_event()
                && key.kind == KeyEventKind::Press
            {
                match key.code {
                    KeyCode::Esc => {
                        self.input = "/ABORT".to_string();
                        self.submit_message()?;
                        break Ok(());
                    }
                    KeyCode::Enter => {
                        if self.input.is_empty() {
                            continue;
                        }
                        let exit = self.input == "/ABORT";
                        if let Some(s) = self.responses.last() {
                            match s.guess_state() {
                                GuessState::Solved | GuessState::GameOver => {
                                    continue;
                                }
                                _ => {}
                            }
                        }
                        self.submit_message()?;
                        if exit {
                            break Ok(());
                        }
                        self.receive_response()?
                    }
                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                    KeyCode::Backspace => self.delete_char(),
                    _ => {}
                }
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(3),
        ]);
        let [help_area, messages_area, input_area] = frame.area().layout(&layout);

        let (msg, style) = (
            vec![
                "Press ".into(),
                "Enter".bold(),
                " to send your guess".into(),
            ],
            Style::default(),
        );
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);

        let responses: Vec<ListItem> = self
            .responses
            .iter()
            .zip(&self.guesses)
            .map(|(m, g)| color_guess(m, g))
            .collect();
        let messages = List::new(responses).block(Block::bordered().title("Guessses"));
        frame.render_widget(messages, messages_area);
    }
}

fn color_guess<'a>(m: &ServerMsg, g: &'a String) -> ListItem<'a> {
    let content = match m.guess_state() {
        GuessState::Error(e) => Line::from(vec![Span::raw(format!("{e}"))]),
        _ => {
            let mut output = vec![];
            for (i, &letter) in g.as_bytes().iter().enumerate().take(5) {
                let p = match m.colors()[i] {
                    Green => Span::styled(
                        String::from_utf8(vec![letter]).unwrap(),
                        Style::default().fg(Color::Green),
                    ),
                    Yellow => Span::styled(
                        String::from_utf8(vec![letter]).unwrap(),
                        Style::default().fg(Color::Yellow),
                    ),
                    Gray => Span::styled(
                        String::from_utf8(vec![letter]).unwrap(),
                        Style::default().fg(Color::Red),
                    ),
                };
                output.push(p);
            }
            Line::from(output)
        }
    };
    ListItem::new(content)
}

// TODO: scroll
// TODO: result screen
