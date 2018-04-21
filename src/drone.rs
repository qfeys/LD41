#[derive(Debug)]
pub struct Drone {
    pub pos: Pos,
    pub rot: f64,
    pub speed: f64
}

impl Drone {
    pub fn new() -> Drone {
        Drone {
            pos: Pos { x: 0.0, y: 0.0 },
            rot: 0.0,
            speed: 5.0
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
