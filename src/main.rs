use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{prelude::*, widgets::*};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();

    app.add_message(Message { username: "User1".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet.".to_string() });
    app.add_message(Message { username: "User2".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string() });
    app.add_message(Message { username: "User3".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string() });
    app.add_message(Message { username: "User4".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string() });
    app.add_message(Message { username: "User5".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet.".to_string() });
    app.add_message(Message { username: "User6".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string() });
    app.add_message(Message { username: "User7".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string() });
    app.add_message(Message { username: "User8".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string() });
    app.add_message(Message { username: "User10".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet.".to_string() });
    app.add_message(Message { username: "User11".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string() });
    app.add_message(Message { username: "User12".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string() });
    app.add_message(Message { username: "User13".to_string(), content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string() });

    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|frame| ui(frame, &mut app))?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}


fn ui(frame: &mut Frame, app: &mut App) {
    let width = frame.size().width as usize;
    let mut messages = app.messages.clone();

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


    frame.render_widget(
        Block::new()
            .borders(Borders::NONE)
            .title("ToIP")
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
    let input_text = "This is a long input text that should wrap to the next line when it reaches the end of the input block. If the text exceeds the available space, it should scroll vertically.";

    let input_paragraph = Paragraph::new(input_text)
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .style(Style::default())
        .wrap(Wrap { trim: true });

    frame.render_widget(input_paragraph, input_layout[0]);
}


#[derive(Clone)]
struct Message {
    username: String,
    content: String,
}

struct App {
    messages: Vec<Message>,
    scroll: ListState,
}

impl App {
    fn new() -> App {
        App {
            messages: vec![],
            scroll: ListState::default(),
        }
    }

    fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        if self.messages.len() > self.scroll.selected().unwrap_or(0) + 1 {
            self.scroll.select(Some(self.messages.len() - 1));
        }
    }
}
