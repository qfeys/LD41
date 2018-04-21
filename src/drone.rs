
use graphics::*;

#[derive(Debug)]
pub struct Drone {
    pub pos: Pos,
    pub rot: f64,
    pub speed: f64,
}

impl Drone {
    pub fn new() -> Drone {
        Drone {
            pos: Pos { x: 0.0, y: 0.0 },
            rot: 0.0,
            speed: 5.0,
        }
    }

    pub fn walk(&mut self, dt: f64) {
        self.pos.x += self.speed * dt * self.rot.cos();
        self.pos.y += self.speed * dt * self.rot.sin();
    }
}

#[derive(Debug)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    // Screen coordinates
    pub fn s_cor(
        &self,
        base_context: Context,
        s_width: u32,
        s_height: u32,
        x_center: f64,
        y_center: f64,
        scale: f64,
    ) -> [[f64; 3]; 2] {
        base_context
            .transform
            .trans(s_width as f64 / 2.0, s_height as f64 / 2.0)
            .trans((self.x - x_center) / scale, (self.y - y_center) / scale)
    }
}
