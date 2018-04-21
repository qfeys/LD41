use opengl_graphics::*;
use graphics::*;

use Pos;
use drone::unit_type::*;

#[derive(Debug)]
pub struct Base {
    pub pos: Pos,
    pub rot: f64,
    prod_queue: Vec<order>,
}

impl Base {
    pub fn new() -> Base {
        Base {
            pos: Pos { x: 0.0, y: 0.0 },
            rot: 0.0,
            prod_queue: Vec::new(),
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

    pub fn queue_worker(&mut self) {
        self.prod_queue.push(order::new_worker());
    }

    pub fn queue_soldier(&mut self) {
        self.prod_queue.push(order::new_soldier());
    }

    pub fn update(&mut self, dt: f64){
        let next = self.prod_queue.get(0);
        let unit = match next {
            Some(order) => order.update(dt),
            None => None
        };
    }
}

#[derive(Debug)]
struct order {
    unit: ::drone::unit_type,
    time_left: f64,
}

impl order {
    fn new_worker() -> order {
        order {
            unit: worker,
            time_left: 4.0,
        }
    }
    fn new_soldier() -> order {
        order {
            unit: soldier,
            time_left: 6.0,
        }
    }
    fn update(&mut self, dt:f64) -> Option<::drone::unit_type>{
        self.time_left-= dt;
        if self.time_left <= 0.0{
            return Some(self.unit);
        }
        None
    }
}
