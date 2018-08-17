use BBox;
use Object;
use object;
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
        obj.transform = object::Transform::new(4.0, 4.0, 8.0, 8.0);
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

        // Updates position based on velocity and handle collisions
        self.obj.update(map);

        let mut beneath_bbox = self.obj.get_bbox();
        beneath_bbox.pos.y += 32.0;

        if !Object::collided(&beneath_bbox, map).is_some() && !self.obj.is_jumping {
            self.obj.is_falling = true;
        }

        self.set_animation();
        Ok(())
    }

    fn set_animation(&mut self) {
        self.sprite_id = PLAYER_SPRITE_STANDING; // Default case is standing animation
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
}
