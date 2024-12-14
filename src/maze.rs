use rand::{thread_rng, Rng};

pub enum Direction {
    Up(Coordinates),
    Down(Coordinates),
    Left(Coordinates),
    Right(Coordinates),
}

impl Direction {
    fn get_coordinates(&self) -> Coordinates {
        match self {
            Direction::Up(c) => c.clone(),
            Direction::Down(c) => c.clone(),
            Direction::Left(c) => c.clone(),
            Direction::Right(c) => c.clone(),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up(c) => Direction::Down(c.down().get_coordinates()),
            Direction::Down(c) => Direction::Up(c.up().get_coordinates()),
            Direction::Left(c) => Direction::Right(c.right().get_coordinates()),
            Direction::Right(c) => Direction::Left(c.left().get_coordinates()),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn up(&self) -> Direction {
        Direction::Up(Coordinates {
            x: self.x,
            y: self.y - 1,
        })
    }

    fn down(&self) -> Direction {
        Direction::Down(Coordinates {
            x: self.x,
            y: self.y + 1,
        })
    }

    fn left(&self) -> Direction {
        Direction::Left(Coordinates {
            x: self.x - 1,
            y: self.y,
        })
    }

    fn right(&self) -> Direction {
        Direction::Right(Coordinates {
            x: self.x + 1,
            y: self.y,
        })
    }

    fn get_random_directions(&self) -> Vec<Direction> {
        let mut directions = vec![self.up(), self.down(), self.left(), self.right()];

        let mut rng = thread_rng();
        for i in 0..3 {
            let j = rng.gen_range(i + 1..4);
            directions.swap(i, j);
        }

        directions
    }
}

#[derive(Default, Clone)]
pub struct Node {
    steps: usize,
    pub coordinates: Coordinates,
    pub up: Option<Coordinates>,
    pub down: Option<Coordinates>,
    pub left: Option<Coordinates>,
    pub right: Option<Coordinates>,
}

impl Node {
    pub fn add_edge(&mut self, direction: Direction) {
        match direction {
            Direction::Up(c) => self.up = Some(c),
            Direction::Down(c) => self.down = Some(c),
            Direction::Left(c) => self.left = Some(c),
            Direction::Right(c) => self.right = Some(c),
        }
    }

    fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.coordinates = coordinates;
    }

    fn has_edges(&self) -> bool {
        self.up.is_some() || self.down.is_some() || self.left.is_some() || self.right.is_some()
    }

    fn set_steps(&mut self, steps: usize) {
        self.steps = steps;
    }

    pub fn get_steps(&self) -> usize {
        self.steps
    }
}

pub struct Maze {
    height: usize,
    width: usize,
    nodes: Vec<Vec<Node>>,
}

impl Maze {
    pub fn new(height: usize, width: usize) -> Self {
        let nodes = vec![vec![Node::default(); width]; height];
        Self {
            height,
            width,
            nodes,
        }
    }

    pub fn nodes(&self) -> &Vec<Vec<Node>> {
        self.nodes.as_ref()
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get_node(&self, coordinates: &Coordinates) -> Option<&Node> {
        self.nodes
            .get(coordinates.y as usize)?
            .get(coordinates.x as usize)
    }

    pub fn get_node_mut(&mut self, coordinates: &Coordinates) -> Option<&mut Node> {
        self.nodes
            .get_mut(coordinates.y as usize)?
            .get_mut(coordinates.x as usize)
    }

    pub fn generate(&mut self, prev: Direction, curr: Coordinates, steps: usize) -> Option<()> {
        let node = self.get_node_mut(&curr)?;
        if node.has_edges() {
            return None;
        }

        node.add_edge(prev);
        node.set_steps(steps);

        for next in curr.get_random_directions() {
            if let Some(_) = self.generate(next.opposite(), next.get_coordinates(), steps + 1) {
                self.get_node_mut(&curr)?.add_edge(next);
            }
        }

        let node = self.get_node_mut(&curr)?;
        node.set_coordinates(curr);

        Some(())
    }
}
