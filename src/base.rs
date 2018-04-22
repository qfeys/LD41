use opengl_graphics::*;
use graphics::*;

use Pos;
use drone::unit_type::*;

#[derive(Debug)]
pub struct Base {
    pub pos: Pos,
    pub rot: f64,
    prod_queue: Vec<Order>,
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
        self.prod_queue.push(Order::new_worker());
        println!("new worker ordered.");
    }

    pub fn queue_soldier(&mut self) {
        self.prod_queue.push(Order::new_soldier());
        println!("new soldier ordered.");
    }

    pub fn update(&mut self, dt: f64) -> Option<::drone::unit_type> {
        if self.prod_queue.is_empty() == false {
            let unit = self.prod_queue[0].update(dt);
            if let Some(_) = unit {
                self.prod_queue.remove(0);
            }
            unit
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Order {
    unit: ::drone::unit_type,
    time_left: f64,
}

impl Order {
    fn new_worker() -> Order {
        Order {
            unit: Worker,
            time_left: 4.0,
        }
    }
    fn new_soldier() -> Order {
        Order {
            unit: Soldier,
            time_left: 6.0,
        }
    }
    fn update(&mut self, dt: f64) -> Option<::drone::unit_type> {
        self.time_left -= dt;
        if self.time_left <= 0.0 {
            return Some(self.unit);
        }
        None
    }
}
