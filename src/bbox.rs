use graphics::Point2;

/// Bounding box used by entities for collision
pub struct BBox {
    pub pos: Point2,
    pub size: Point2,
}

impl BBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> BBox {
        BBox {
            pos: Point2::new(x, y),
            size: Point2::new(width, height),
        }
    }

    /// Checks if bounding box intersects with another bounding box
    pub fn intersects(&self, other: &BBox) -> bool {
        (self.pos.x + self.size.x > other.pos.x && self.pos.x <= other.pos.x + other.size.x)
            && (self.pos.y + self.size.y > other.pos.y && self.pos.y <= other.pos.y + other.size.y)
    }
}
