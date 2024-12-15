use crate::game::Game;
use crate::maze::Coordinates;
use crate::{view, GameEvent};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io::Stdout;
use std::thread::sleep;
use std::time::Duration;

const GAME_WATCH_PAUSE_DURATION: Duration = Duration::from_millis(20);

pub fn play_game(
    mut game: Game,
    mut terminal: Terminal<CrosstermBackend<Stdout>>,
) -> Result<bool, Box<dyn Error>> {
    let mut player_won = false;

    loop {
        terminal.draw(|frame| view::render(frame, &game))?;

        match crate::event_listener() {
            GameEvent::MovePlayer(direction) => {
                if game.move_player(direction).is_err() {
                    continue;
                }
            },
            GameEvent::AutoMove => game.auto_move()?,
            GameEvent::ToggleBreadcrumbs => game.toggle_breadcrumbs(),
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

pub fn watch_game(
    mut game: Game,
    mut terminal: Terminal<CrosstermBackend<Stdout>>,
) -> Result<bool, Box<dyn Error>> {
    let node = game.maze().get_node(game.player()).unwrap();
    let prev = node.get_coordinates().clone();

    let moves: Vec<Coordinates> = [node.left(), node.down(), node.right()]
        .into_iter()
        .flatten()
        .cloned()
        .collect();

    for next_move in moves {
        if recursive_move(&mut game, &prev, &next_move, &mut terminal) {
            break;
        }
    }

    Ok(true)
}

fn recursive_move(
    game: &mut Game,
    prev: &Coordinates,
    curr: &Coordinates,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> bool {
    if curr == game.end() {
        return true;
    }

    update_player_view(curr.clone(), game, terminal);

    let node = game.maze().get_node(curr).unwrap();

    let mut next_moves: Vec<Coordinates> = [node.up(), node.right(), node.down(), node.left()]
        .into_iter()
        .flatten()
        .filter(|&c| c != prev)
        .cloned()
        .collect();

    while let Some(next_move) = next_moves.pop() {
        if recursive_move(game, curr, &next_move, terminal) {
            return true;
        }
        update_player_view(curr.clone(), game, terminal);
    }

    false
}

fn update_player_view(
    coordinates: Coordinates,
    game: &mut Game,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) {
    game.move_player_coordinates(coordinates);
    terminal.draw(|frame| view::render(frame, game)).unwrap();
    // slow down so the user can see the player move
    sleep(GAME_WATCH_PAUSE_DURATION);
}
