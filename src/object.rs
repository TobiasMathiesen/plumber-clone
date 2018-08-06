/// Object underlying other entities (position, bounding box, physics)
use ggez::graphics::Point2;

#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
}

pub struct Object {
    pub pos: Point2,
    pub bounds: Point2,
    pub velocity: Point2,
    pub is_falling: bool,
    pub is_jumping: bool,
    pub is_running: bool,
    pub direction: Direction,
}

const DEFAULT_DIRECTION: Direction = Direction::Right;

impl Object {
    pub fn new() -> Object {
        Object {
            pos: Point2::new(0.0, 0.0),
            bounds: Point2::new(0.0, 0.0),
            velocity: Point2::new(0.0, 0.0),
            is_falling: false,
            is_jumping: false,
            is_running: false,
            direction: DEFAULT_DIRECTION,
        }
    }
}
