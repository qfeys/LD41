use opengl_graphics::*;
use graphics::*;

use Pos;

#[derive(Debug)]
pub struct Base {
    pub pos: Pos,
    pub rot: f64,
}

impl Base {
    pub fn new() -> Base {
        Base {
            pos: Pos { x: 0.0, y: 0.0 },
            rot: 0.0,
        }
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
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 0.8];
        let square = rectangle::square(0.0, 0.0, 10.0);
        let transform = self.pos
            .s_cor(c, s_width, s_height, x_center, y_center, scale);
        rectangle(BLUE, square, transform, gl);
    }
}
