use std::{env, io::{self, stdout}, sync::{Arc, Mutex}};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{prelude::*, widgets::*};
use udp_toip::{ToipClient, ToipServer};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut args_iter = args.iter().skip(1); // Skip the program name
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "--c" => {
                if let (Some(addr), Some(username)) = (args_iter.next(), args_iter.next()) {
                    println!("Starting client with address: {} and username: {}", addr, username);
                    start_client(addr.clone(), username.clone())?;
                } else {
                    panic!("Expected an address and a username after '--c'");
                }
            },
            "--s" => {
                if let Some(addr) = args_iter.next() {
                    println!("Running in server mode with address: {}", addr);

                    let server = ToipServer::new(addr.clone());
                    
                    // init server
                    server.init()?;
                } else {
                    panic!("Expected an address after '--s'");
                }
            },
            "--help" => {
                println!("Modes \n Client mode: --c <server address> <username> \n Server mode: --s <server address> \n Client example: cargo run toip-tui --c 0.0.0.0:0 username \n Server example: Client example: cargo run toip-tui --c 0.0.0.0:0");
                return Ok(()); // Assuming the function returns a Result<(), YourErrorType>
            },
            _ => println!("Unknown argument. Use --help for more info"),
        }
    }


    Ok(())
}

fn start_client(address: String, username: String) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();

    let client = ToipClient::new(address, username);
    client.init()?;

    let mut prev_message_buffer_size = 0;

    loop {
        let current_message_buffer_size = client.message_buffer.lock().unwrap().clone().len();
        if current_message_buffer_size > prev_message_buffer_size {
            terminal.flush()?;
            prev_message_buffer_size = current_message_buffer_size;
        }

        terminal.draw(|frame| ui(frame, &mut app, client.message_buffer.clone()))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => app.submit_message(&client)?,
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Left => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right => {
                        app.move_cursor_right();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                InputMode::Editing => {}
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn ui(
    frame: &mut Frame, 
    app: &mut App, 
    messages: Arc<Mutex<Vec<udp_toip::Message>>>
) {
    let mut messages = messages.lock().unwrap();
    let width = frame.size().width as usize;

    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(6),
        ],
    )
    .spacing(1)
    .split(frame.size());

    let title = format!("ToIP {}", messages.len());

    frame.render_widget(
        Block::new()
            .borders(Borders::NONE)
            .title(title)
            .title_style(Style::new().green()),
        main_layout[0],
    );

    let mut message_block_heights = messages
        .iter()
        .map(|m| {
            textwrap::wrap(&m.content, width).len() as u16 + 2
        })
        .collect::<Vec<u16>>();
    
    let mut total_block_height: u16 = message_block_heights.iter().sum();
    let message_layout_height = main_layout[1].height;

    while total_block_height > message_layout_height {
        // remove first
        message_block_heights.remove(0);
        messages.remove(0);
        total_block_height = message_block_heights.iter().sum();
    }

    let messages_layout = Layout::new(
            Direction::Vertical, 
            message_block_heights.iter().map(|&c| Constraint::Length(c)),
        )
        .flex(layout::Flex::End)
        .split(main_layout[1]);

    for (i, m) in messages.iter().enumerate() {
        let message_p = Paragraph::new(m.content.clone())
            .block(Block::default().borders(Borders::ALL).title(m.username.clone()))
            .style(Style::default())
            .wrap(Wrap { trim: true });

        frame.render_widget(message_p, messages_layout[i]);
    }

    let input_layout = Layout::new(Direction::Vertical, [Constraint::Min(0)]).split(main_layout[2]);

    let input_paragraph = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title("Input (esc > q to exit)"))
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .wrap(Wrap { trim: true });

    frame.render_widget(input_paragraph, input_layout[0]);

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => {
            frame.set_cursor(
                input_layout[0].x + app.cursor_position as u16 + 1,
                input_layout[0].y + 1,
            );
        }
    }
}

enum InputMode {
    Normal,
    Editing,
}

struct App {
    input: String,
    cursor_position: usize,
    input_mode: InputMode
}

impl App {
    fn new() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Editing,
            cursor_position: 0
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    fn submit_message(&mut self, client: &ToipClient) -> std::io::Result<()> {
        client.send_message(self.input.clone())?;
        self.input.clear();
        self.reset_cursor();

        Ok(())
    }
}
