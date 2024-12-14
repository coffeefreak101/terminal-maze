mod game;
mod maze;
mod view;

use crate::game::{Game, MoveDirection};
use clap::Parser;
use crossterm::event::Event::Key;
use crossterm::event::KeyCode;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use maze::Maze;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io::{stdout, Stdout};

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn teardown_terminal() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

enum GameEvent {
    Quit,
    MovePlayer(MoveDirection),
    AutoMove,
    Error(Box<dyn Error>),
}

fn event_listener() -> GameEvent {
    loop {
        return match crossterm::event::read() {
            Ok(Key(key_event)) => match key_event.code {
                KeyCode::Up => GameEvent::MovePlayer(MoveDirection::Up),
                KeyCode::Down => GameEvent::MovePlayer(MoveDirection::Down),
                KeyCode::Left => GameEvent::MovePlayer(MoveDirection::Left),
                KeyCode::Right => GameEvent::MovePlayer(MoveDirection::Right),
                KeyCode::Char('q') | KeyCode::Esc => GameEvent::Quit,
                KeyCode::Char(' ') => GameEvent::AutoMove,
                _ => continue,
            },
            Err(err) => GameEvent::Error(err.into()),
            _ => continue,
        };
    }
}

fn play_game(
    mut game: Game,
    mut terminal: Terminal<CrosstermBackend<Stdout>>,
) -> Result<bool, Box<dyn Error>> {
    let mut player_won = false;

    loop {
        terminal.draw(|frame| view::render(frame, &game))?;

        match event_listener() {
            GameEvent::MovePlayer(direction) => {
                if game.move_player(direction).is_err() {
                    continue;
                }
            },
            GameEvent::AutoMove => game.auto_move()?,
            GameEvent::Quit => break,
            GameEvent::Error(err) => {
                return Err(err);
            },
        }

        if game.player() == game.end() {
            player_won = true;
            break;
        }
    }

    Ok(player_won)
}

#[derive(Parser, Debug)]
#[command(about = "Terminal maze game")]
struct Config {
    #[arg(long = "height", default_value = "10")]
    height: usize,
    #[arg(long = "width", default_value = "10")]
    width: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    setup_terminal()?;

    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;

    let maze = Maze::new(config.height, config.width);
    let game = Game::new(maze);

    let result = play_game(game, terminal);

    teardown_terminal()?;

    let success = result?;

    if success {
        println!("You win!");
    }

    Ok(())
}
