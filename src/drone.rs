use opengl_graphics::*;
use graphics::*;

use Pos;

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

    pub fn draw(
        &self,
        gl: &mut GlGraphics,
        c: &Context,
        s_width: u32,
        s_height: u32,
        x_center: f64,
        y_center: f64,
        scale: f64,
    ) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square = rectangle::square(0.0, 0.0, 3.5);
        let transform = self.pos
            .s_cor(c, s_width, s_height, x_center, y_center, scale);
        rectangle(RED, square, transform, gl);
    }
}
