use crate::maze::{Coordinates, Direction, Maze};
use rand::{thread_rng, Rng};
use std::error::Error;

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    maze: Maze,
    player_coordinates: Coordinates,
    end_coordinates: Coordinates,
}

impl Game {
    pub fn new(mut maze: Maze) -> Self {
        let width = maze.width();
        let height = maze.height();

        let mut rng = thread_rng();
        let start = Coordinates::new(rng.gen_range(0..width) as i32, 0);
        let end = Coordinates::new(rng.gen_range(0..width) as i32, (height - 1) as i32);

        maze.generate(Direction::Down(end.clone()), end.clone(), 0);
        maze.get_node_mut(&start)
            .unwrap()
            .add_edge(Direction::Up(start.clone()));
        Self {
            maze,
            player_coordinates: start,
            end_coordinates: end,
        }
    }

    pub fn maze(&self) -> &Maze {
        &self.maze
    }

    pub fn player(&self) -> &Coordinates {
        &self.player_coordinates
    }

    pub fn end(&self) -> &Coordinates {
        &self.end_coordinates
    }

    pub fn move_player(&mut self, direction: MoveDirection) -> Result<(), Box<dyn Error>> {
        let curr_node = self
            .maze
            .get_node(self.player())
            .ok_or("player is outside of the maze")?;

        let next_coordinates = match direction {
            MoveDirection::Up => curr_node.up.as_ref(),
            MoveDirection::Down => curr_node.down.as_ref(),
            MoveDirection::Left => curr_node.left.as_ref(),
            MoveDirection::Right => curr_node.right.as_ref(),
        }
        .ok_or("cannot move player in that direction")?;

        self.player_coordinates = next_coordinates.clone();

        Ok(())
    }

    pub fn auto_move(&mut self) -> Result<(), Box<dyn Error>> {
        let curr_node = self
            .maze
            .get_node(self.player())
            .ok_or("player is outside of the maze")?;

        let mut min_steps = curr_node.get_steps();
        let mut next_move = Coordinates::default();

        for coordinates in [
            curr_node.up.as_ref(),
            curr_node.down.as_ref(),
            curr_node.left.as_ref(),
            curr_node.right.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            let next_node = self.maze.get_node(coordinates);
            let steps = next_node.unwrap().get_steps();
            if steps < min_steps {
                min_steps = steps;
                next_move = coordinates.clone();
            }
        }

        self.player_coordinates = next_move;

        Ok(())
    }
}
