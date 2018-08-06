use ggez::graphics;
use graphics::Point2;

pub const PLAYER_SPRITE_OFFSET: (f32, f32) = (80.0, 2.0);
pub const SPRITES_PER_ROW: usize = 33;
pub const SPRITE_SIZE: f32 = 16.0;
pub const SCALE_FACTOR: f32 = 2.0;
pub const EMPTY_SPRITE: usize = 100000;

/// Gets the location of the sprite based on its ID
pub fn id_to_player_sprite(index: usize) -> Point2 {
    let x = PLAYER_SPRITE_OFFSET.0 + ((index % 21 as usize) as f32 * 17.0);
    let y = PLAYER_SPRITE_OFFSET.1 + ((index / 21 as usize) as f32 * 16.0);
    Point2::new(x, y)
}

/// Creates the parameters needed to draw the sprite
pub fn create_sprite_param(
    index: usize,
    dest: graphics::Point2,
    image: &graphics::Image,
    player_sprite: bool,
) -> graphics::DrawParam {
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
        src: graphics::Rect::fraction(
            x as f32,
            y as f32,
            SPRITE_SIZE,
            SPRITE_SIZE,
            &image.get_dimensions(),
        ),
        dest,
        rotation: 0.0,
        scale: graphics::Point2::new(SCALE_FACTOR, SCALE_FACTOR),
        offset: graphics::Point2::new(0.0, 0.0),
        ..Default::default()
    }
}
