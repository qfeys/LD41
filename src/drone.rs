use opengl_graphics::*;
use graphics::*;

use Pos;

#[derive(Debug)]
pub struct Drone {
    pub pos: Pos,
    pub rot: f64,
    pub speed: f64,
    pub is_selected: bool,
    target_pos: Pos,
    u_type: unit_type,
}

impl Drone {
    pub fn new() -> Drone {
        Drone {
            pos: Pos { x: 0.0, y: 0.0 },
            rot: 0.0,
            speed: 25.0,
            is_selected: false,
            target_pos: Pos { x: 100.0, y: 000.0 },
            u_type: unit_type::worker,
        }
    }
    pub fn from_pos(pos: Pos) -> Drone {
        Drone {
            pos,
            rot: 0.0,
            speed: 25.0,
            is_selected: false,
            target_pos: pos,
            u_type: unit_type::worker,
        }
    }

    pub fn walk(&mut self, dt: f64) {
        use std::f64::consts::PI;
        let pi2 = PI * 2.0;
        let dir = self.target_pos - self.pos;
        let angle = f64::atan2(dir.y, dir.x);
        let diff = (((angle - self.rot) % pi2) + pi2) % pi2;
        if diff < PI {
            if diff > PI * 2.0 - dt {
                self.rot += diff;
            } else {
                self.rot += dt;
            }
        } else {
            if diff < dt {
                self.rot -= diff;
            } else {
                self.rot -= dt
            }
        }
        self.rot = ((self.rot % pi2) + pi2) % pi2;
        self.pos.x += self.speed * dt * self.rot.cos();
        self.pos.y += self.speed * dt * self.rot.sin();
    }

    pub fn set_destination(&mut self, destination: Pos) {
        self.target_pos = destination;
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
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 0.8];
        let square = rectangle::square(0.0, 0.0, 3.5);
        let transform = self.pos
            .s_cor(c, s_width, s_height, x_center, y_center, scale);
        if self.is_selected {
            let big_square = rectangle::square(-1.0, -1.0, 5.5);
            rectangle(BLUE, big_square, transform, gl);
        }
        rectangle(RED, square, transform, gl);
    }
}

#[derive(Debug, Copy, Clone)]
pub enum unit_type {
    worker,
    soldier,
}
