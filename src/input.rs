use ggez::event::{Keycode, Mod};
use ggez::Context;
use object::Direction;
use sprite::EMPTY_SPRITE;
use state::MainState;

impl MainState {
    pub fn key_down_editor(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        let map = &mut self.map.as_mut().unwrap();
        let max_index = (map.dimensions.0 * map.dimensions.1) as usize;
        let tile = &mut map.tiles[self.editor.index].id;
        match keycode {
            Keycode::D => {
                self.editor.index += 1;
                self.editor.index %= max_index;
            }
            Keycode::A => {
                if self.editor.index > 0 {
                    self.editor.index -= 1;
                }
            }
            Keycode::S => {
                self.editor.index += map.dimensions.0 as usize;
                self.editor.index %= map.dimensions.0 as usize * map.dimensions.1 as usize;
            }
            Keycode::W => {
                let mut index = self.editor.index as isize - map.dimensions.0 as isize;
                if index < 0 {
                    index += max_index as isize;
                }
                self.editor.index = index as usize;
            }
            Keycode::Up => {
                *tile += 1;
                if *tile >= EMPTY_SPRITE {
                    *tile = 0;
                }
            }
            Keycode::Down => {
                if *tile > 0 {
                    *tile -= 1;
                }
                if *tile >= 1000 {
                    *tile = 0;
                }
            }
            Keycode::X => {
                *tile = EMPTY_SPRITE;
            }
            _ => {}
        };
    }

    pub fn key_down_play(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        repeat: bool,
    ) {
        if repeat {
            return;
        }

        match keycode {
            Keycode::D => {
                self.player.moving_right = true;
                self.player.obj.direction = Direction::Right;
                if self.player.obj.velocity.x <= -3.0 {
                    self.player.turn_cycle = 20;
                }
            }
            Keycode::A => {
                self.player.moving_left = true;
                self.player.obj.direction = Direction::Left;
                if self.player.obj.velocity.x >= 3.0 {
                    self.player.turn_cycle = 20;
                }
            }
            Keycode::Space => {
                if !self.player.obj.is_jumping && !self.player.obj.is_falling {
                    self.player.obj.is_jumping = true;
                    let run_speed = self.player.obj.velocity.x.abs() / 4.0;
                    self.player.obj.velocity.y = 6.0 + 1.5 * run_speed;
                }
            }
            Keycode::LShift => {
                self.player.obj.is_running = true;
            }
            _ => {}
        }
    }

    pub fn key_up_play(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        repeat: bool,
    ) {
        if repeat {
            return;
        }

        match keycode {
            Keycode::D => {
                self.player.moving_right = false;
            }
            Keycode::A => {
                self.player.moving_left = false;
            }
            Keycode::Space => {
                if self.player.obj.is_jumping {
                    self.player.obj.is_jumping = false;
                    self.player.obj.is_falling = true;
                    self.player.obj.velocity.y = 0.0;
                }
            }
            Keycode::LShift => {
                self.player.obj.is_running = false;
            }
            _ => {}
        }
    }
}
