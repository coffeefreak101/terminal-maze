mod game;
mod maze;
mod mode;
mod view;

use crate::game::{Game, MoveDirection};
use clap::{Parser, ValueEnum};
use crossterm::event::Event::Key;
use crossterm::event::KeyCode;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use maze::Maze;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io::{stdout, Stdout};

enum GameEvent {
    Quit,
    MovePlayer(MoveDirection),
    AutoMove,
    ToggleBreadcrumbs,
    Error(Box<dyn Error>),
}

#[derive(Clone, Debug, ValueEnum)]
pub enum GameMode {
    Play,
    Watch,
}

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

fn event_listener() -> GameEvent {
    loop {
        return match crossterm::event::read() {
            Ok(Key(key_event)) => match key_event.code {
                KeyCode::Up => GameEvent::MovePlayer(MoveDirection::Up),
                KeyCode::Down => GameEvent::MovePlayer(MoveDirection::Down),
                KeyCode::Left => GameEvent::MovePlayer(MoveDirection::Left),
                KeyCode::Right => GameEvent::MovePlayer(MoveDirection::Right),
                KeyCode::Char('q') | KeyCode::Esc => GameEvent::Quit,
                KeyCode::Char('b') => GameEvent::ToggleBreadcrumbs,
                KeyCode::Char(' ') => GameEvent::AutoMove,
                _ => continue,
            },
            Err(err) => GameEvent::Error(err.into()),
            _ => continue,
        };
    }
}

#[derive(Parser, Debug)]
#[command(about = "Terminal maze game")]
struct Config {
    #[arg(long = "height", default_value = "10")]
    height: usize,

    #[arg(long = "width", default_value = "10")]
    width: usize,

    #[arg(
        short = 'b',
        long = "breadcrumbs",
        help = "Show red dots along your path"
    )]
    breadcrumbs: bool,

    #[arg(
        short = 'm',
        long = "mode",
        value_enum,
        default_value = "play",
        help = "Watch the game solve the maze"
    )]
    mode: GameMode,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    setup_terminal()?;

    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;

    let maze = Maze::new(config.height, config.width);
    let game = Game::new(maze, config.breadcrumbs);

    let result = match config.mode {
        GameMode::Play => mode::play_game(game, terminal),
        GameMode::Watch => mode::watch_game(game, terminal),
    };

    teardown_terminal()?;

    let success = result?;

    if success {
        println!("You win!");
    }

    Ok(())
}
