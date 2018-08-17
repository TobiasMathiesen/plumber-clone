/// Object underlying other entities (position, bounding box, physics)
use ggez::graphics::Point2;
use sprite::EMPTY_SPRITE;
use state::Map;
use BBox;

#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
}

pub struct Transform {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Transform {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Transform {
        Transform {
            x,
            y,
            width,
            height,
        }
    }
}

pub struct Object {
    pub pos: Point2,
    pub bounds: Point2,
    pub transform: Transform,
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
            transform: Transform::new(0.0, 0.0, 0.0, 0.0),
            velocity: Point2::new(0.0, 0.0),
            is_falling: false,
            is_jumping: false,
            is_running: false,
            direction: DEFAULT_DIRECTION,
        }
    }

    // Tests if the player collided with any active, non-empty tiles
    // If collision occured returns the bounding box of the tile collided with
    pub fn collided(bbox: &BBox, map: &Map) -> Option<BBox> {
        for (i, tile) in map.tiles.iter().enumerate() {
            if tile.id == EMPTY_SPRITE {
                continue;
            }

            let i = i as u32;
            let x = (i % map.dimensions.0) * 32;
            let y = (i / map.dimensions.0) * 32;
            let other_bbox = BBox::new(x as f32, y as f32, 32.0, 32.0);
            if bbox.intersects(&other_bbox) {
                return Some(other_bbox);
            }
        }
        None
    }

    // Get bounding box for object
    pub fn get_bbox(&self) -> BBox {
        let mut bbox = BBox {
            pos: self.pos,
            size: self.bounds,
        };
        self.apply_transform(bbox)
    }

    // Apply transformation to a BBox (used to make BBox smaller / larger than sprite)
    fn apply_transform(&self, mut bbox: BBox) -> BBox {
        bbox.pos.x += self.transform.x;
        bbox.size.x -= self.transform.width;
        bbox.pos.y += self.transform.y;
        bbox.size.y -= self.transform.height;
        bbox
    }

    pub fn update(&mut self, map: &Map) {
        // Apply velocity and test for collision and correct accordingly
        self.pos.y -= self.velocity.y;
        let bbox = self.get_bbox();
        self.handle_collision_y(&bbox, map);
        self.pos.x += self.velocity.x;
        let bbox = self.get_bbox();
        self.handle_collision_x(&bbox, map);
    }

    fn handle_collision_x(&mut self, bbox: &BBox, map: &Map) {
        if let Some(collision) = Object::collided(bbox, map) {
            if self.velocity.x < 0.0 {
                self.pos.x = collision.pos.x + collision.size.x - 3.0;
            } else {
                self.pos.x = collision.pos.x - self.bounds.x + 3.0;
            }
            self.velocity.x = 0.0;
        }
    }

    fn handle_collision_y(&mut self, bbox: &BBox, map: &Map) {
        if let Some(collision) = Object::collided(bbox, map) {
            if self.is_falling {
                self.pos.y = collision.pos.y - self.bounds.y - 1.0;
                self.is_falling = false;
            } else if self.is_jumping {
                self.pos.y = collision.pos.y + collision.size.y + 1.0;
                self.is_jumping = false;
                self.is_falling = true;
            }
            self.velocity.y = 0.0;
        }
    }
}
