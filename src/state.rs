use ggez::graphics::{Color, Point2};
use ggez::{event, graphics, Context, GameResult};
use object::Direction;
use player::Player;
use enemy::Enemy;
use sprite;
use sprite::{EMPTY_SPRITE, SCALE_FACTOR, SPRITE_SIZE};

#[derive(PartialEq)]
pub enum GameMode {
    Play,
    Editor,
}

pub struct Editor {
    pub index: usize,
}

pub struct Tile {
    pub active: bool,
    pub id: usize,
}

pub struct Map {
    pub dimensions: (u32, u32),
    pub tiles: Vec<Tile>,
}

pub struct MainState {
    pub tile_image: graphics::Image,
    pub player_image: graphics::Image,
    pub map: Option<Map>,
    pub mode: GameMode,
    pub editor: Editor,
    pub player: Player,
    pub enemies: Vec<Enemy>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut tile_image = graphics::Image::new(ctx, "/tiles.png")?;
        let mut player_image = graphics::Image::new(ctx, "/player.png")?;
        tile_image.set_filter(graphics::FilterMode::Nearest);
        player_image.set_filter(graphics::FilterMode::Nearest);
        let mut map = Map {
            dimensions: (16, 16),
            tiles: Vec::new(),
        };
        let mode = GameMode::Editor;
        let editor = Editor { index: 0 };
        let player = Player::new();
        let enemies = Vec::new();
        for _ in 0..16 * 16 {
            map.tiles.push(Tile {
                active: true,
                id: EMPTY_SPRITE,
            });
        }
        let main_state = MainState {
            tile_image,
            map: Some(map),
            mode,
            editor,
            player_image,
            player,
            enemies,
        };
        Ok(main_state)
    }

    fn update_editor(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw_player(&mut self, ctx: &mut Context) -> GameResult<()> {
        let id = self.player.sprite_id;
        let dest = self.player.obj.pos;
        let mut param = sprite::create_sprite_param(id, dest, &self.player_image, true);
        if self.player.obj.direction == Direction::Left {
            param.scale.x *= -1.0;
            param.dest.x += 32.0;
        }
        graphics::draw_ex(ctx, &self.player_image, param)?;
        Ok(())
    }

    fn draw_enemies(&mut self, ctx: &mut Context) -> GameResult<()> {
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
            let param = sprite::create_sprite_param(id, dest, &self.tile_image, false);
            graphics::draw_ex(ctx, &self.tile_image, param)?;
            graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;
        }

        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.mode {
            GameMode::Editor => {
                self.update_editor(ctx)?;
            }
            GameMode::Play => {
                self.player.update(self.map.as_ref().unwrap())?;
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

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::Keycode,
        keymod: event::Mod,
        repeat: bool,
    ) {
        match keycode {
            event::Keycode::F1 => {
                if self.mode == GameMode::Editor {
                    self.mode = GameMode::Play;
                } else {
                    self.mode = GameMode::Editor;
                }
            },
            event::Keycode::R => {
                self.player.obj.pos = Point2::new(0.0, 320.0);
            }
            _ => {}
        }
        match self.mode {
            GameMode::Editor => {
                self.key_down_editor(ctx, keycode, keymod, repeat);
            }
            GameMode::Play => {
                self.key_down_play(ctx, keycode, keymod, repeat);
            }
        }
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::Keycode,
        keymod: event::Mod,
        repeat: bool,
    ) {
        if repeat {
            return;
        }
        match self.mode {
            GameMode::Editor => {}
            GameMode::Play => {
                self.key_up_play(ctx, keycode, keymod, repeat);
            }
        }
    }
}
