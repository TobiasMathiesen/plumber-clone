use BBox;
use Object;

use ggez::GameResult;
use graphics::Point2;
use sprite::EMPTY_SPRITE;
use state::Map;

pub struct Player {
    pub obj: Object,
    pub sprite_id: usize,
    pub moving_left: bool,
    pub moving_right: bool,
    pub run_cycle: usize,
    pub turn_cycle: usize,
}

pub const PLAYER_SPRITE_STANDING: usize = 42;
pub const RUN_MODIFIER: f32 = 1.5; // Speed modifier when running

impl Player {
    pub fn new() -> Player {
        let mut obj = Object::new();
        obj.bounds = Point2::new(32.0, 32.0);
        obj.pos = Point2::new(0.0, 320.0);
        let sprite_id = PLAYER_SPRITE_STANDING;
        let moving_left = false;
        let moving_right = false;
        let run_cycle = 0;
        let turn_cycle = 0;
        Player {
            obj,
            sprite_id,
            moving_left,
            moving_right,
            run_cycle,
            turn_cycle,
        }
    }

    pub fn update(&mut self, map: &Map) -> GameResult<()> {
        if self.obj.velocity.y > 0.0 {
            self.obj.velocity.y -= 0.2;
            if self.obj.velocity.y < 0.0 {
                self.obj.is_jumping = false;
                self.obj.is_falling = true;
            }
        }

        if self.obj.is_falling {
            self.obj.velocity.y -= 0.2;
            if self.obj.velocity.y <= -5.0 {
                self.obj.velocity.y = -5.0;
            }
        }

        if self.moving_right {
            self.obj.velocity.x += 0.1;
            if self.obj.is_running {
                if self.obj.velocity.x > 4.0 * RUN_MODIFIER {
                    self.obj.velocity.x = 4.0 * RUN_MODIFIER;
                }
            } else {
                if self.obj.velocity.x > 4.0 {
                    self.obj.velocity.x = 4.0;
                }
            }
        } else if self.obj.velocity.x > 0.0 {
            self.obj.velocity.x -= 0.15;
            if self.obj.velocity.x < 0.0 {
                self.obj.velocity.x = 0.0;
            }
        }

        if self.moving_left {
            self.obj.velocity.x -= 0.1;
            if self.obj.is_running {
                if self.obj.velocity.x < -4.0 * RUN_MODIFIER {
                    self.obj.velocity.x = -4.0 * RUN_MODIFIER;
                }
            } else {
                if self.obj.velocity.x < -4.0 {
                    self.obj.velocity.x = -4.0;
                }
            }
        } else if self.obj.velocity.x < 0.0 {
            self.obj.velocity.x += 0.15;
            if self.obj.velocity.x > 0.0 {
                self.obj.velocity.x = 0.0;
            }
        }

        self.obj.pos.y -= self.obj.velocity.y;

        let player_bbox = self.get_bbox();

        if let Some(collision) = self.collided(&player_bbox, map) {
            if self.obj.is_falling {
                self.obj.pos.y = collision.pos.y - self.obj.bounds.y - 1.0;
                self.obj.is_falling = false;
            } else if self.obj.is_jumping {
                self.obj.pos.y = collision.pos.y + collision.size.y + 1.0;
                self.obj.is_jumping = false;
                self.obj.is_falling = true;
            }
            self.obj.velocity.y = 0.0;
        }

        self.obj.pos.x += self.obj.velocity.x;

        let player_bbox = self.get_bbox();

        if let Some(collision) = self.collided(&player_bbox, map) {
            if self.obj.velocity.x < 0.0 {
                self.obj.pos.x = collision.pos.x + collision.size.x - 3.0;
            } else {
                self.obj.pos.x = collision.pos.x - self.obj.bounds.x + 3.0;
            }
            self.obj.velocity.x = 0.0;
        }

        let mut beneath_bbox = self.get_bbox();
        beneath_bbox.pos.y += 32.0;

        if !self.collided(&beneath_bbox, map).is_some() && !self.obj.is_jumping {
            self.obj.is_falling = true;
        }

        self.set_animation();
        Ok(())
    }

    fn set_animation(&mut self) {
        self.sprite_id = PLAYER_SPRITE_STANDING; // Default case is the standing animation
        if self.obj.is_jumping || self.obj.is_falling {
            self.sprite_id = 47;
        } else if self.obj.velocity.x != 0.0 {
            // If player recently turned, show turning animation
            if self.turn_cycle > 0 {
                self.sprite_id = 46;
                self.turn_cycle -= 1;
                return;
            }

            self.sprite_id = match self.run_cycle {
                0...5 => 43,
                5...10 => 45,
                _ => 44,
            };
            self.run_cycle += 1;
            if self.run_cycle >= 15 {
                self.run_cycle = 0;
            }
        }
    }

    // Tests if the player collided with any active, non-empty tiles
    // If collision occured returns the bounding box of the tile collided with
    fn collided(&mut self, player_bbox: &BBox, map: &Map) -> Option<BBox> {
        for (i, tile) in map.tiles.iter().enumerate() {
            if tile.id == EMPTY_SPRITE {
                continue;
            }

            let i = i as u32;
            let x = (i % map.dimensions.0) * 32;
            let y = (i / map.dimensions.0) * 32;
            let bbox = BBox::new(x as f32, y as f32, 32.0, 32.0);
            if player_bbox.intersects(&bbox) {
                return Some(bbox);
            }
        }
        None
    }

    pub fn get_bbox(&self) -> BBox {
        let mut bbox = BBox {
            pos: self.obj.pos,
            size: self.obj.bounds,
        };

        // Bounding box isn't as big as whole sprite, so trim by 4 pixels on each side
        bbox.pos.x += 4.0;
        bbox.size.x -= 8.0;
        bbox.pos.y += 4.0;
        bbox.size.y -= 8.0;
        bbox
    }
}
