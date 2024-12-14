use crate::game::Game;
use crate::maze::{Coordinates, Node};
use ratatui::prelude::{Color, Marker};
use ratatui::widgets::canvas::{Canvas, Line, Painter, Shape};
use ratatui::widgets::Block;
use ratatui::Frame;

#[derive(Clone, Default, Debug)]
struct ScreenLocation {
    x: f64,
    y: f64,
}

impl ScreenLocation {
    fn new(x: f64, y: f64) -> Self {
        ScreenLocation { x, y }
    }
}

pub struct CellBuilder {
    cell_width: f64,
    cell_height: f64,
    color: Color,
    start: ScreenLocation,
}

impl CellBuilder {
    fn new(cell_height: f64, cell_width: f64, color: Color, start: ScreenLocation) -> Self {
        Self {
            cell_height,
            cell_width,
            color,
            start,
        }
    }

    fn location_from_coordinates(&self, coordinates: &Coordinates) -> ScreenLocation {
        ScreenLocation {
            x: self.start.x + (coordinates.x as f64 * self.cell_width),
            y: self.start.y - (coordinates.y as f64 * self.cell_height),
        }
    }

    fn build_cell(&self, node: &Node) -> Cell {
        let location = self.location_from_coordinates(&node.coordinates);
        Cell {
            x: location.x,
            y: location.y,
            height: self.cell_height,
            width: self.cell_width,
            color: self.color,
            top: node.up.is_none(),
            bottom: node.down.is_none(),
            left: node.left.is_none(),
            right: node.right.is_none(),
        }
    }

    fn build_player(&self, coordinates: &Coordinates) -> Player {
        let mut location = self.location_from_coordinates(coordinates);
        location.x += self.cell_width / 2.0;
        location.y -= self.cell_height / 2.0;
        Player::new(location, Color::Cyan)
    }
}

/// The cell is positioned from its top left corner.
#[derive(Debug, Default, Clone, PartialEq)]
struct Cell {
    /// The `x` position of the cell.
    pub x: f64,
    /// The `y` position of the cell.
    pub y: f64,
    /// The width of the cell.
    pub width: f64,
    /// The height of the cell.
    pub height: f64,
    /// The color of the cell.
    pub color: Color,
    /// Sides of the cell.
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Shape for Cell {
    fn draw(&self, painter: &mut Painter) {
        if self.left {
            Line {
                x1: self.x,
                y1: self.y,
                x2: self.x,
                y2: self.y - self.height,
                color: self.color,
            }
            .draw(painter)
        };
        if self.top {
            Line {
                x1: self.x,
                y1: self.y,
                x2: self.x + self.width,
                y2: self.y,
                color: self.color,
            }
            .draw(painter);
        }
        if self.right {
            Line {
                x1: self.x + self.width,
                y1: self.y,
                x2: self.x + self.width,
                y2: self.y - self.height,
                color: self.color,
            }
            .draw(painter);
        }
        if self.bottom {
            Line {
                x1: self.x,
                y1: self.y - self.height,
                x2: self.x + self.width,
                y2: self.y - self.height,
                color: self.color,
            }
            .draw(painter);
        }
    }
}

struct Player {
    location: ScreenLocation,
    color: Color,
}

impl Player {
    fn new(location: ScreenLocation, color: Color) -> Self {
        Self { location, color }
    }
}

impl Shape for Player {
    fn draw(&self, painter: &mut Painter) {
        if let Some((x, y)) = painter.get_point(self.location.x, self.location.y) {
            painter.paint(x, y, self.color);
        }
    }
}

pub fn render(frame: &mut Frame, game: &Game) {
    let area = frame.size();
    let left = 0.0;
    let right = f64::from(area.width);
    let bottom = 0.0;
    let top = f64::from(area.height).mul_add(2.0, -4.0);
    let center = ScreenLocation::new(right / 2f64, top / 2f64);
    let cell_width = 4.0;
    let cell_height = 4.0;
    let table_width = game.maze().width() as f64 * cell_width;
    let table_height = game.maze().height() as f64 * cell_height;
    let top_left = ScreenLocation::new(
        center.x - (table_width / 2.0),
        center.y - (table_height / 2.0) + table_height,
    );
    let builder = CellBuilder::new(cell_height, cell_width, Color::LightGreen, top_left);

    let canvas = Canvas::default()
        .block(Block::bordered().title("Terminal Maze"))
        .marker(Marker::HalfBlock)
        .x_bounds([left, right])
        .y_bounds([bottom, top])
        .paint(|ctx| {
            for row in game.maze().nodes() {
                for node in row.iter() {
                    ctx.draw(&builder.build_cell(node));
                }
            }
            ctx.draw(&builder.build_player(game.player()))
        });

    frame.render_widget(canvas, frame.size());
}
