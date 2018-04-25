use opengl_graphics::*;
use graphics::*;
use std::sync::atomic::{self, AtomicUsize};
use map::*;
use gsd::*;

static OBJECT_COUNTER: AtomicUsize = atomic::ATOMIC_USIZE_INIT;
const MAX_CARGO: f64 = 1.0;
const GATHER_RATE: f64 = 0.2;

use Pos;

#[derive(Debug)]
pub struct Drone {
    pub id: usize,
    pub pos: Pos,
    pub rot: f64,
    pub speed: f64,
    pub is_selected: bool,
    pub team: u8,
    pub u_type: unit_type,
    behaviour: Behaviour,
}

impl Drone {
    pub fn new() -> Drone {
        Drone {
            id: OBJECT_COUNTER.fetch_add(1, atomic::Ordering::SeqCst),
            pos: Pos { x: 0.0, y: 0.0 },
            rot: 0.0,
            speed: 15.0,
            is_selected: false,
            team: 0,
            u_type: unit_type::Worker { cargo: 0.0 },
            behaviour: Behaviour::Move(Pos { x: 100.0, y: 000.0 }),
        }
    }
    pub fn from_pos_n_type(pos: Pos, typ: unit_type) -> Drone {
        match typ {
            unit_type::Worker { cargo: _ } => Drone {
                id: OBJECT_COUNTER.fetch_add(1, atomic::Ordering::SeqCst),
                pos,
                rot: 0.0,
                speed: 15.0,
                is_selected: false,
                team: 0,
                u_type: unit_type::Worker { cargo: 0.0 },
                behaviour: Behaviour::Move(pos),
            },
            unit_type::Soldier => Drone {
                id: OBJECT_COUNTER.fetch_add(1, atomic::Ordering::SeqCst),
                pos,
                rot: 0.0,
                speed: 25.0,
                is_selected: false,
                team: 0,
                u_type: unit_type::Soldier,
                behaviour: Behaviour::Move(pos),
            },
        }
    }

    pub fn update(&mut self, dt: f64, map: &mut Map, gsd: &mut GameStateData) {
        match self.behaviour {
            Behaviour::Move(destination) => {
                self.walk(dt, destination);
                if (self.pos - destination).mag() < 2.0 {
                    match self.u_type {
                        unit_type::Worker { cargo: _ } => {
                            self.behaviour = Behaviour::Gather(destination)
                        }
                        unit_type::Soldier => self.behaviour = Behaviour::Attack(destination),
                    }
                }
            }
            Behaviour::Gather(location) => {
                self.walk(dt / 4.0, location);
                match self.u_type {
                    unit_type::Worker { ref mut cargo } => {
                        *cargo += map.gather_resource(location, GATHER_RATE, dt);
                        if *cargo >= MAX_CARGO {
                            *cargo = MAX_CARGO;
                            self.behaviour = Behaviour::ReturnTb(location);
                        }
                    }
                    unit_type::Soldier => panic!("Invalid behaviour: Soldiers can not Gather"),
                }
            }
            Behaviour::ReturnTb(prev_loc) => {
                let t = self.team as usize;
                self.walk(dt, gsd.base_locations[t]);
                if self.pos.mag() < 2.0 {
                    match self.u_type {
                        unit_type::Worker { ref mut cargo } => {
                            gsd.deposite_resource(*cargo, self.team);
                            *cargo = 0.0;
                            self.behaviour = Behaviour::ReturnGathering(prev_loc);
                        }
                        unit_type::Soldier => self.behaviour = Behaviour::Idle,
                    }
                }
            }
            Behaviour::ReturnGathering(destination) => {
                self.walk(dt, destination);
                if (self.pos - destination).mag() < 2.0 {
                    self.behaviour = Behaviour::Gather(destination);
                }
            }
            Behaviour::Attack(loc) => self.walk(dt, loc),
            Behaviour::Evade(ref d_box, ref beh_box) => {}
            Behaviour::Persue(ref d_box, ref beh_box) => {}
            Behaviour::Idle => (),
        }
    }

    fn walk(&mut self, dt: f64, destination: Pos) {
        use std::f64::consts::PI;
        let pi2 = PI * 2.0;
        let dir = destination - self.pos;
        let angle = f64::atan2(dir.y, dir.x);
        let diff = (((angle - self.rot) % pi2) + pi2) % pi2;
        let max_turn = dt * 2.0;
        if diff < PI {
            if diff > PI * 2.0 - max_turn {
                self.rot += diff;
            } else {
                self.rot += max_turn;
            }
        } else {
            if diff < max_turn {
                self.rot -= diff;
            } else {
                self.rot -= max_turn
            }
        }
        self.rot = ((self.rot % pi2) + pi2) % pi2;
        self.pos.x += self.speed * dt * self.rot.cos();
        self.pos.y += self.speed * dt * self.rot.sin();
    }

    pub fn set_destination(&mut self, destination: Pos) {
        self.behaviour = Behaviour::Move(destination);
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
    Worker { cargo: f64 },
    Soldier,
}

#[derive(Debug)]
enum Behaviour {
    Idle,
    Move(Pos),
    Gather(Pos),
    ReturnTb(Pos),
    ReturnGathering(Pos),
    Attack(Pos),
    Evade(Box<Drone>, Box<Behaviour>),
    Persue(Box<Drone>, Box<Behaviour>),
}
