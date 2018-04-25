use opengl_graphics::*;
use graphics::*;

use Pos;
use drone::unit_type::*;
use gsd::*;

const DISTANCE_BASES: f64 = 400.0;
const WORKER_COST: f64 = 1.0;
const SOLDIER_COST: f64 = 1.0;

#[derive(Debug)]
pub struct Base {
    pub pos: Pos,
    pub prod_queue: Vec<Order>,
    pub team: u8,
}

impl Base {
    pub fn new() -> Base {
        Base {
            pos: Pos { x: 0.0, y: 0.0 },
            prod_queue: Vec::new(),
            team: 1,
        }
    }

    pub fn all_new() -> Vec<Base> {
        use std::f64::consts::PI;
        let mut v = Vec::new();
        let radius = DISTANCE_BASES / (2.0 * f64::sin(PI / ::NUM_OF_PLAYERS as f64));
        for team in 0..::NUM_OF_PLAYERS {
            let pos = Pos {
                x: radius * f64::cos(team as f64 / ::NUM_OF_PLAYERS as f64 * 2.0 * PI),
                y: radius * f64::sin(team as f64 / ::NUM_OF_PLAYERS as f64 * 2.0 * PI),
            };
            v.push(Base {
                pos,
                prod_queue: Vec::new(),
                team: team as u8,
            });
        }
        v
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

    pub fn queue_worker(&mut self, gsd: &mut GameStateData) {
        if gsd.allocate_resource(WORKER_COST, self.team) {
            self.prod_queue.push(Order::new_worker());
        }
    }

    pub fn queue_soldier(&mut self, gsd: &mut GameStateData) {
        if gsd.allocate_resource(SOLDIER_COST, self.team) {
            self.prod_queue.push(Order::new_soldier());
        }
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
pub struct Order {
    pub unit: ::drone::unit_type,
    pub time_left: f64,
}

impl Order {
    fn new_worker() -> Order {
        Order {
            unit: Worker { cargo: 0.0 },
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
