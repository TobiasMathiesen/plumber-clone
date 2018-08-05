extern crate ggez;

use ggez::graphics::spritebatch;
use ggez::conf;
use ggez::event;
use ggez::event::{Keycode, Mod};
use ggez::graphics;
use ggez::graphics::{Color, Point2};
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use std::f32;

const SCREEN_WIDTH: u32 = 512;
const SCREEN_HEIGHT: u32 = 512;
const SCALE_FACTOR: f32 = 2.0;
const SPRITE_SIZE: f32 = 16.0;
const SPRITES_PER_ROW: usize = 33;
const EMPTY_SPRITE: usize = 100000;
const PLAYER_SPRITE_OFFSET: (f32, f32) = (80.0, 2.0);
const RUN_MODIFIER: f32 = 1.5;

#[derive(PartialEq)]
enum GameMode {
    Play,
    Editor
}

struct Editor {
    index: usize,
}

struct Tile {
    active: bool,
    id: usize
}

struct Map {
    dimensions: (u32, u32),
    tiles: Vec<Tile>,
}

struct MainState {
    tile_image: graphics::Image,
    player_image: graphics::Image,
    map: Option<Map>,
    mode: GameMode,
    editor: Editor,
    player: Player
}

struct Player {
    obj: Object,
    sprite_id: usize,
    moving_left: bool,
    moving_right: bool,
    run_cycle: usize,
    turn_cycle: usize,
}

impl Player {
    fn new() -> Player {
        let mut obj = Object::new();
        obj.bounds = Point2::new(32.0, 32.0);
        obj.pos = Point2::new(0.0, 320.0);
        let sprite_id = 42;
        let moving_left = false;
        let moving_right = false;
        let run_cycle = 0;
        let turn_cycle = 0;
        Player { obj, sprite_id, moving_left, moving_right, run_cycle, turn_cycle }
    }

    fn get_bbox(&self) -> BBox {
        let mut bbox = BBox {
            pos: self.obj.pos,
            size:  self.obj.bounds
        };

        bbox.pos.x += 4.0;
        bbox.size.x -= 8.0;
        bbox.pos.y += 4.0;
        bbox.size.y -= 8.0;
        bbox
    }
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right
}

struct Object {
    pos: Point2,
    bounds: Point2,
    velocity: Point2,
    is_falling: bool,
    is_jumping: bool,
    is_running: bool,
    direction: Direction,
}

impl Object {
    fn new() -> Object {
        Object {
            pos: Point2::new(0.0, 0.0),
            bounds: Point2::new(0.0, 0.0),
            velocity: Point2::new(0.0, 0.0),
            is_falling: false,
            is_jumping: false,
            is_running: false,
            direction: Direction::Right
        }
    }
}

// Bounding Box
struct BBox {
    pos: Point2,
    size: Point2,
}

impl BBox {
    fn new(x: f32, y: f32, width: f32, height: f32) -> BBox {
        BBox {
            pos: Point2::new(x, y),
            size: Point2::new(width, height)
        }
    }

    fn intersects(&self, other: &BBox) -> bool {
        (self.pos.x + self.size.x > other.pos.x && self.pos.x <= other.pos.x + other.size.x) && (self.pos.y + self.size.y > other.pos.y && self.pos.y <= other.pos.y + other.size.y)
    }
}

fn id_to_player_sprite(index: usize) -> Point2 {
    let x = PLAYER_SPRITE_OFFSET.0 + ((index % 21 as usize) as f32 * 17.0);
    let y = PLAYER_SPRITE_OFFSET.1 + ((index / 21 as usize) as f32 * 16.0);
    Point2::new(x, y)
}

fn get_sprite_param(index: usize, dest: graphics::Point2, image: &graphics::Image, player_sprite: bool) 
    -> graphics::DrawParam {
    let (x, y);
    if player_sprite {
        let offset = id_to_player_sprite(index);
        x = offset.x as u32;
        y = offset.y as u32;
    } else {
        x = (SPRITE_SIZE * (index % SPRITES_PER_ROW) as f32) as u32;
        y = (SPRITE_SIZE * (index / SPRITES_PER_ROW) as f32) as u32;
    }
    graphics::DrawParam {
        src: graphics::Rect::fraction(x as f32, 
                                      y as f32, 
                                      SPRITE_SIZE,
                                      SPRITE_SIZE, 
                                      &image.get_dimensions()),
        dest,
        rotation: 0.0,
        scale: graphics::Point2::new(SCALE_FACTOR, SCALE_FACTOR),
        offset: graphics::Point2::new(0.0, 0.0),
        ..Default::default()
    }
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut tile_image = graphics::Image::new(ctx, "/tiles.png")?;
        let mut player_image = graphics::Image::new(ctx, "/player.png")?;
        tile_image.set_filter(graphics::FilterMode::Nearest);
        player_image.set_filter(graphics::FilterMode::Nearest);
        let mut map = Map { dimensions: (16, 16), tiles: Vec::new() };
        let mode = GameMode::Editor;
        let editor = Editor { index: 0 };
        let player = Player::new();
        for _ in (0..16*16) {
            map.tiles.push(Tile { active: true, id: EMPTY_SPRITE });
        }
        let main_state = MainState { tile_image, map: Some(map), mode, editor, player_image, player };
        Ok(main_state)
    }

    fn update_editor(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(()) 
    }

    fn set_animation(&mut self) {
        self.player.sprite_id = 42;
        if self.player.obj.is_jumping || self.player.obj.is_falling { 
            self.player.sprite_id = 47;
        } else if self.player.obj.velocity.x != 0.0 {
            // If player recently turned, show turning animation
            if self.player.turn_cycle > 0 {
                self.player.sprite_id = 46;
                self.player.turn_cycle -= 1;
                return;
            }

            self.player.sprite_id = match self.player.run_cycle {
                0 ... 5 => 43,
                5 ... 10 => 45,
                _ => 44,
            };
            self.player.run_cycle += 1;
            if self.player.run_cycle >= 15 {
                self.player.run_cycle = 0;
            }
        }
    }

    fn update_player(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.player.obj.velocity.y > 0.0 {
            self.player.obj.velocity.y -= 0.2;
            if self.player.obj.velocity.y < 0.0 {
                self.player.obj.is_jumping = false;
                self.player.obj.is_falling = true;
            }
        }

        if self.player.obj.is_falling {
            self.player.obj.velocity.y -= 0.2;
            if self.player.obj.velocity.y <= -5.0 {
                self.player.obj.velocity.y = -5.0;
            }
        }

        if self.player.moving_right {
            self.player.obj.velocity.x += 0.1;
            if self.player.obj.is_running {
                if self.player.obj.velocity.x > 4.0 * RUN_MODIFIER {
                    self.player.obj.velocity.x = 4.0 * RUN_MODIFIER;
                }
            } else {
                if self.player.obj.velocity.x > 4.0 {
                    self.player.obj.velocity.x = 4.0;
                }
            }
            
        } else if self.player.obj.velocity.x > 0.0 {
            self.player.obj.velocity.x -= 0.15;
            if self.player.obj.velocity.x < 0.0 {
                self.player.obj.velocity.x = 0.0;
            }
        }

        if self.player.moving_left {
            self.player.obj.velocity.x -= 0.1;
            if self.player.obj.is_running {
                if self.player.obj.velocity.x < -4.0 * RUN_MODIFIER {
                    self.player.obj.velocity.x = -4.0 * RUN_MODIFIER;
                } 
            } else {
                if self.player.obj.velocity.x < -4.0 {
                    self.player.obj.velocity.x = -4.0;
                } 
            }
                } else if self.player.obj.velocity.x < 0.0 {
            self.player.obj.velocity.x += 0.15;
            if self.player.obj.velocity.x > 0.0 {
                self.player.obj.velocity.x = 0.0;
            }
        }

        self.player.obj.pos.y -= self.player.obj.velocity.y;

        let player_bbox = self.player.get_bbox();

        if let Some(collision) = self.collided(&player_bbox) {
            if self.player.obj.is_falling {
                self.player.obj.pos.y = collision.pos.y - self.player.obj.bounds.y - 1.0;
                self.player.obj.is_falling = false;
            } else if self.player.obj.is_jumping {
                self.player.obj.pos.y = collision.pos.y + collision.size.y + 1.0;
                self.player.obj.is_jumping = false;
                self.player.obj.is_falling = true;
            }
                self.player.obj.velocity.y = 0.0;
        }

        self.player.obj.pos.x += self.player.obj.velocity.x;

        let player_bbox = self.player.get_bbox();

        if let Some(collision) = self.collided(&player_bbox) {
            if self.player.obj.velocity.x < 0.0 {
                self.player.obj.pos.x = collision.pos.x + collision.size.x - 3.0;
            } else {
                self.player.obj.pos.x = collision.pos.x - self.player.obj.bounds.x + 3.0;
            }
            self.player.obj.velocity.x = 0.0;
        }

        let mut beneath_bbox = self.player.get_bbox();
        beneath_bbox.pos.y += 32.0;

        if !self.collided(&beneath_bbox).is_some() && !self.player.obj.is_jumping {
            self.player.obj.is_falling = true;
        }

        self.set_animation();
        Ok(())
    }

    fn collided(&mut self, player_bbox: &BBox) -> Option<BBox> {
        let mut map = &mut self.map.as_mut().unwrap();
        for (i, tile) in map.tiles.iter().enumerate() {
            if tile.id == EMPTY_SPRITE {
                continue;
            }

            let i = i as u32;
            let x = (i % map.dimensions.0) * 32;
            let y = (i / map.dimensions.0) * 32;
            let bbox = BBox::new(x as f32, y as f32, 32.0, 32.0);
            if (player_bbox.intersects(&bbox)) {
                return Some(bbox)
            }
        }
        None
    }

    fn key_down_editor(&mut self, ctx: &mut Context, keycode: event::Keycode, keymod: event::Mod, repeat: bool) {
        let mut map = &mut self.map.as_mut().unwrap();
        let max_index = (map.dimensions.0 * map.dimensions.1) as usize;
        let mut tile = &mut map.tiles[self.editor.index].id; 
        match keycode {
            event::Keycode::D => {
                self.editor.index += 1;
                self.editor.index %= max_index;
            },
            event::Keycode::A => {
                if self.editor.index > 0 {
                    self.editor.index -= 1;
                }
            },
            event::Keycode::S => {
                self.editor.index += map.dimensions.0 as usize;
                self.editor.index %= map.dimensions.0 as usize * map.dimensions.1 as usize;
            },
            event::Keycode::W => {
                let mut index = self.editor.index as isize - map.dimensions.0 as isize;
                if index < 0 {
                    index += max_index as isize;
                }
                self.editor.index = index as usize;
            }
            event::Keycode::Up => {
                *tile += 1;
                if *tile >= EMPTY_SPRITE {
                    *tile = 0;
                }
            },
            event::Keycode::Down => {
                if *tile > 0 {
                    *tile -= 1;
                }
                if *tile >= 1000 {
                    *tile = 0;
                }
            },
            event::Keycode::X => {
                *tile = EMPTY_SPRITE;
            }
            _ => { }
        };
    }

    fn key_down_play(&mut self, ctx: &mut Context, keycode: event::Keycode, keymod: event::Mod, repeat: bool) {
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
            },
            Keycode::A => {
                self.player.moving_left = true;
                self.player.obj.direction = Direction::Left;
                if self.player.obj.velocity.x >= 3.0 {
                    self.player.turn_cycle = 20;
                }
            },
            Keycode::Space => {
                if !self.player.obj.is_jumping && !self.player.obj.is_falling {
                    self.player.obj.is_jumping = true;
                    let run_speed = self.player.obj.velocity.x.abs() / 4.0;
                    self.player.obj.velocity.y = 6.0 + 1.5 * run_speed;
                }
            },
            Keycode::LShift => {
                self.player.obj.is_running = true;
            }
            _ => { }
        }
    }

    fn key_up_play(&mut self, ctx: &mut Context, keycode: event::Keycode, keymod: event::Mod, repeat: bool) {
        if repeat {
            return;
        }

        match keycode {
            Keycode::D => {
                self.player.moving_right = false;
            },
            Keycode::A => {
                self.player.moving_left = false;
            },
            Keycode::Space => {
                if self.player.obj.is_jumping {
                    self.player.obj.is_jumping = false;
                    self.player.obj.is_falling = true;
                    self.player.obj.velocity.y = 0.0;
                }
            },
            Keycode::LShift => {
                self.player.obj.is_running = false;
            }
            _ => { }
        }
    }

    fn draw_player(&mut self, ctx: &mut Context) -> GameResult<()> {
        let id = self.player.sprite_id;
        let mut dest = self.player.obj.pos;
        let mut param = get_sprite_param(id, dest, &self.player_image, true);
        if self.player.obj.direction == Direction::Left {
            param.scale.x *= -1.0;
            param.dest.x += 32.0;
        }
        graphics::draw_ex(ctx, &self.player_image, param)?;
        Ok(())
    }

    fn draw_map(&mut self, ctx: &mut Context) -> GameResult<()> {
        let map = &self.map.as_ref().unwrap().tiles;
        for (i, tile) in map.iter().enumerate() {
            let tile_size = SPRITE_SIZE * SCALE_FACTOR;
            let mut id = tile.id;
            if id == EMPTY_SPRITE {
                if i == self.editor.index {
                    graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 0.2))?;
                    id = 0;
                } else {
                    continue;
                }
            }
 
            let dimensions = &self.map.as_ref().unwrap().dimensions;
            let x = (i as u32 % dimensions.0) as f32 * tile_size;
            let y = (i as u32 / dimensions.0) as f32 * tile_size;
            let dest = Point2::new(x, y);
            let param = get_sprite_param(id, dest, &self.tile_image, false);
            graphics::draw_ex(ctx, &self.tile_image, param)?;
            graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        }

        Ok(())
    }

    fn draw_editor(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
} 

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        
        match self.mode {
            GameMode::Editor => {
                self.update_editor(ctx)?;
            },
            GameMode::Play => {
                self.update_player(ctx)?;
            }
        };
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, Color::new(0.43, 0.56, 0.97, 1.0));
        self.draw_map(ctx)?; 
        self.draw_player(ctx)?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, 
                      ctx: &mut Context,
                      keycode: event::Keycode,
                      keymod: event::Mod,
                      repeat: bool) {
        match keycode {
            event::Keycode::F1 => {
                if self.mode == GameMode::Editor {
                    self.mode = GameMode::Play;
                } else {
                    self.mode = GameMode::Editor;
                }
            },
            _ => { }
        }
        match self.mode {
            GameMode::Editor => { 
                self.key_down_editor(ctx, keycode, keymod, repeat);
            },
            GameMode::Play => {
                self.key_down_play(ctx, keycode, keymod, repeat);
            },
            _ => { }

        }
    }

    fn key_up_event(&mut self, 
                      ctx: &mut Context,
                      keycode: event::Keycode,
                      keymod: event::Mod,
                      repeat: bool) {
        if repeat {
            return;
        }
        match self.mode {
            GameMode::Editor => { 

            },
            GameMode::Play => {
                self.key_up_play(ctx, keycode, keymod, repeat);
            },
            _ => { }

        }
    } 
}

fn main() {
    let c = conf::Conf::new();
    println!("Starting with default config: {:#?}", c);
    let ctx = &mut ggez::ContextBuilder::new("mario", "skuzzi")
        .window_setup(ggez::conf::WindowSetup::default().title("Mario"))
        .window_mode(ggez::conf::WindowMode::default()
        .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .expect("Failed to build ggez context");
	

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    } 
}
