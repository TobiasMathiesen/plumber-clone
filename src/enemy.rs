use ggez::graphics::{Point2};
use BBox;
use Object;
use object::Direction;
use state::Map;

use ggez::GameResult;

pub enum EnemyType {
    GOOMBA
}

pub struct Enemy {
    pub obj: Object,
    pub enemy_type: EnemyType
}

impl Enemy {
    pub fn new_goomba() -> Enemy {
        let mut obj = Object::new();
        obj.bounds = Point2::new(16.0, 16.0);
        obj.pos = Point2::new(0.0, 0.0);
        let enemy_type = EnemyType::GOOMBA;

        Enemy { obj, enemy_type }
    }

    pub fn update(&mut self, map: &Map) -> GameResult<()> {
        match self.enemy_type {
            EnemyType::GOOMBA => self.update_goomba(map),
        }
    }

    fn update_goomba(&mut self, map: &Map) -> GameResult<()> {
        match self.obj.direction {
            Direction::Left => {
                self.obj.velocity.x -= 0.1;  
            },
            Direction::Right => {
                self.obj.velocity.x += 0.1;
            }
        }
        Ok(())   
    }
}
