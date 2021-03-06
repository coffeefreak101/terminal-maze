
extern crate rand;
extern crate termion;

use std::io::{
    Read,
    Write,
    stdin,
    stdout
};
use rand::{thread_rng, Rng};
use std::thread::sleep_ms;
use std::rc::Rc;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{
    clear,
    cursor,
    event::Key,
};


const USER_CHAR: char = '\u{263b}';
const WALL_CHAR: char = '\u{2588}';
const SPACE_CHAR: char = ' ';


#[derive(Clone)]
#[derive(Debug)]
struct Coordinates {
    x: isize,
    y: isize
}


impl Coordinates {
    fn up(&self, count: isize) -> Coordinates {
        Coordinates { x: self.x, y: self.y-count }
    }

    fn down(&self, count: isize) -> Coordinates {
        Coordinates { x: self.x, y: self.y+count }
    }

    fn left(&self, count: isize) -> Coordinates {
        Coordinates { x: self.x-count, y: self.y }
    }

    fn right(&self, count: isize) -> Coordinates {
        Coordinates { x: self.x+count, y: self.y }
    }
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Coordinates) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone)]
struct BoardSpace {
    coordinates: Coordinates,
    console_location: cursor::Goto,
    steps_from_finish: usize,
}

impl BoardSpace {
    fn new(coordinates: Coordinates, steps_from_finish: usize) -> BoardSpace {
        BoardSpace {
            console_location: cursor::Goto((coordinates.x as u16)+1, (coordinates.y as u16)+2),
            coordinates: coordinates,
            steps_from_finish
        }
    }
}

impl PartialEq for BoardSpace {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}


fn make_odd(i: usize) -> usize {
    return if i % 2 == 0 {
        i + 1
    } else {
        i
    }
}


struct MazeGame {
    height: usize,
    width: usize,
    board: Vec<Vec<Option<Rc<BoardSpace>>>>,
    start: Rc<BoardSpace>,
    end: Rc<BoardSpace>,
    user_space: Rc<BoardSpace>
}

impl MazeGame {
    fn new(mut width: usize, mut height: usize) -> MazeGame {
        width = make_odd(width);
        height = make_odd(height);

        let mut rng = thread_rng();
        let mut start_x = rng.gen_range(1, width-1);
        let mut end_x = rng.gen_range(1, width-1);

        start_x = make_odd(start_x);
        end_x = make_odd(end_x);

        let board = vec![vec![None; width]; height];

        let start_space = Rc::new(BoardSpace::new(Coordinates { x: start_x as isize, y: 0 }, height*width));
        let end_space = Rc::new(BoardSpace::new(Coordinates { x: end_x as isize, y: (height as isize)-1 }, 0));

        MazeGame {
            height,
            width,
            board: board,
            start: start_space.clone(),
            end: end_space,
            user_space: start_space
        }
    }

    fn print_maze(&self) {
        print!("{}", cursor::Hide);
        println!("{}", cursor::Goto(1, 0));

        for row in self.board.iter() {
            let mut row_icons = Vec::new();
            for space in row.iter() {
                row_icons.push(self.get_char_for_space(space));
            }
            let row_string: String = row_icons.iter().collect();
            println!("{}", row_string);
        }
        stdout().flush().unwrap();
    }

    fn get_char_for_space(&self, optional_space: &Option<Rc<BoardSpace>>) -> char {
        match optional_space {
            None => WALL_CHAR,
            Some(space) if space.coordinates == self.user_space.coordinates => USER_CHAR,
            Some(_) => SPACE_CHAR
        }
    }

    fn update_user_location(&mut self, space: Rc<BoardSpace>) {
        print!("{}{}", self.user_space.console_location, SPACE_CHAR);
        print!("{}{}", space.console_location, USER_CHAR);
        self.user_space = space;
        stdout().flush().unwrap();
    }

    fn get_space(&self, coordinates: &Coordinates) -> Result<Option<&Rc<BoardSpace>>, String> {
        let err_msg = format!("Invalid coordinates {:?}", coordinates);
        if self.is_valid_coordinates(coordinates) == false {
            return Err(err_msg);
        }
        let row = self.board.get(coordinates.y as usize);
        match row {
            Some(row) => {
                match row.get(coordinates.x as usize) {
                    Some(space) => Ok(space.as_ref()),
                    None => Err(err_msg)
                }
            },
            None => Err(err_msg)
        }
    }

    fn is_valid_coordinates(&self, coordinates: &Coordinates) -> bool {
        return if 0 < coordinates.x || (coordinates.x as usize) < self.width || coordinates.y > 0 || (coordinates.y as usize) < self.height {
            true
        } else {
            false
        }
    }

    fn is_valid_space(&self, coordinates: &Coordinates) -> bool {
        let space = self.get_space(coordinates);
        space.is_ok() && space.unwrap().is_some()
    }

    fn get_random_directions(&self, current_coordinates: &Coordinates) -> Vec<(Coordinates, Coordinates)> {
        let mut directions = vec!(
            (current_coordinates.up(1), current_coordinates.up(2)),
            (current_coordinates.down(1), current_coordinates.down(2)),
            (current_coordinates.left(1), current_coordinates.left(2)),
            (current_coordinates.right(1), current_coordinates.right(2))
        );
        let mut random_directions = Vec::new();
        let mut rng = thread_rng();

        while directions.len() > 0 {
            let i = rng.gen_range(0, directions.len());
            let direction = directions.remove(i);
            if self.is_valid_coordinates(&direction.1) {
                random_directions.push(direction);
            }
        }

        random_directions
    }

    fn set_space(&mut self, space: Rc<BoardSpace>) -> Option<&Rc<BoardSpace>> {
        let (x, y) = (space.coordinates.x as usize, space.coordinates.y as usize);
        self.board[y][x] = Some(space);
        self.board[y][x].as_ref()
    }

    fn set_new_space(&mut self, coordinates: &Coordinates, steps: usize) -> Option<&Rc<BoardSpace>> {
        let new_space = Rc::new(BoardSpace::new(coordinates.clone(), steps));
        self.set_space(new_space)
    }

    fn make_next_move(&mut self, current_coordinates: &Coordinates, steps: usize) {
        let random_directions = self.get_random_directions(current_coordinates);

        for direction in random_directions.iter() {
            let (wall, new_coordinates) = direction;
            let test_space = self.get_space(&new_coordinates);

            if test_space.is_err() || test_space.unwrap() != None {
                continue;
            }

            self.set_new_space(wall, steps+1);
            self.set_new_space(new_coordinates, steps+2);

            self.print_maze();
            sleep_ms(10);

            self.make_next_move(new_coordinates, steps+2);
        }
    }

    fn generate_maze(&mut self) {
        self.set_space(self.start.clone());
        self.set_space(self.end.clone());

        // Building the maze in reverse seems to generate better mazes
        let first_move_coordinates = self.end.coordinates.up(1);

        self.set_new_space(&first_move_coordinates, self.end.steps_from_finish+1);
        self.make_next_move(&first_move_coordinates, self.end.steps_from_finish+1);
    }

    fn auto_move(&self) -> Result<Option<&Rc<BoardSpace>>, String> {
        let directions = vec!(
            self.user_space.coordinates.up(1),
            self.user_space.coordinates.down(1),
            self.user_space.coordinates.left(1),
            self.user_space.coordinates.right(1)
        );

        let spaces: Vec<&Rc<BoardSpace>> = directions.iter()
            .filter(|c| self.is_valid_space(c))
            .map(|c| self.get_space(c).unwrap().unwrap())
            .collect();

        let mut min_space: Option<&Rc<BoardSpace>> = None;

        for space in spaces.iter() {
            if min_space.is_none() || min_space.unwrap().steps_from_finish > space.steps_from_finish {
                min_space = Some(space);
            }
        }

        Ok(min_space)
    }

    pub fn play(&mut self) {
        self.generate_maze();
        self.print_maze();

        let std_in = stdin();
        let mut std_out = stdout().into_raw_mode().unwrap();

        for key in std_in.keys() {
            let new_user_location = match key.unwrap() {
                Key::Up => self.get_space(&self.user_space.coordinates.up(1)),
                Key::Down => self.get_space(&self.user_space.coordinates.down(1)),
                Key::Left => self.get_space(&self.user_space.coordinates.left(1)),
                Key::Right => self.get_space(&self.user_space.coordinates.right(1)),
                Key::Char('q') | Key::Esc | Key::Ctrl('c') => { break; },
                Key::Char(' ') => { self.auto_move() }
                _ => { continue; }
            };

            std_out.flush().unwrap();

            if let Ok(Some(new_space)) = new_user_location {
                let new_space = new_space.clone();
                self.update_user_location(new_space);

                if self.user_space == self.end {
                    println!("{}You win!", self.end.console_location);
                    break;
                }
            }

        }
        println!("{}{}", cursor::Goto(1, (self.end.coordinates.y+4) as u16), cursor::Show);
    }
}


fn main() {
    println!("{}", clear::All);

    let mut game = MazeGame::new(50, 25);

    game.play();
}
