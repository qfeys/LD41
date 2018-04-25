use opengl_graphics::*;
use graphics::*;
use std::sync::atomic::{self, AtomicUsize};
use map::*;
use gsd::*;
use std::f64::*;
use Pos;

mod steering;

static OBJECT_COUNTER: AtomicUsize = atomic::ATOMIC_USIZE_INIT;
const MAX_CARGO: f64 = 1.0;
const GATHER_RATE: f64 = 0.2;

#[derive(Debug)]
pub struct Drone {
    pub id: usize,
    pub pos: Pos,
    pub vel: Pos, // Using pos as velcoity vector
    pub max_speed: f64,
    pub max_force: f64,
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
            vel: Pos { x: 0.0, y: 0.0 },
            max_speed: 15.0,
            max_force: 10.0,
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
                pos: Pos { x: 0.0, y: 0.0 },
                vel: Pos { x: 0.0, y: 0.0 },
                max_speed: 15.0,
                max_force: 10.0,
                is_selected: false,
                team: 0,
                u_type: unit_type::Worker { cargo: 0.0 },
                behaviour: Behaviour::Move(pos),
            },
            unit_type::Soldier => Drone {
                id: OBJECT_COUNTER.fetch_add(1, atomic::Ordering::SeqCst),
                pos: Pos { x: 0.0, y: 0.0 },
                vel: Pos { x: 0.0, y: 0.0 },
                max_speed: 25.0,
                max_force: 10.0,
                is_selected: false,
                team: 0,
                u_type: unit_type::Soldier,
                behaviour: Behaviour::Move(pos),
            },
        }
    }

    pub fn update(
        &mut self,
        dt: f64,
        map: &mut Map,
        gsd: &mut GameStateData,
        drone_list: &Vec<(usize, Pos, u8)>,
    ) {
        match self.behaviour {
            Behaviour::Move(destination) => {
                let st = steering::seek(self, destination);
                let ms =self.max_speed;
                self.step(st,ms, dt);
                if (self.pos - destination).mag() < 5.0 {
                    match self.u_type {
                        unit_type::Worker { cargo: _ } => {
                            self.behaviour = Behaviour::Gather(destination, Pos { x: 0.0, y: 0.0 })
                        }
                        unit_type::Soldier => self.behaviour = Behaviour::Attack(destination),
                    }
                }
            }
            Behaviour::Gather(location, mut wander) => {
                //steering::walk(self, dt / 4.0, location);
                match self.u_type {
                    unit_type::Worker { ref mut cargo } => {
                        *cargo += map.gather_resource(self.pos, GATHER_RATE, dt);
                        if *cargo >= MAX_CARGO {
                            *cargo = MAX_CARGO;
                            self.behaviour = Behaviour::ReturnTb(location);
                        }
                    }
                    unit_type::Soldier => panic!("Invalid behaviour: Soldiers can not Gather"),
                }
                let st = steering::wander(self, 6.0, 18.0, &mut wander, dt);
                if let Behaviour::Gather(_, ref mut wan) = self.behaviour {
                    *wan = wander;
                }
                let ms =self.max_speed/3.0;
                self.step(st,ms, dt);
            }
            Behaviour::ReturnTb(prev_loc) => {
                let t = self.team as usize;
                let st = steering::seek(self, gsd.base_locations[t]);
                let ms =self.max_speed;
                self.step(st,ms, dt);
                if (self.pos - gsd.base_locations[t]).mag() < 8.0 {
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
                let st = steering::seek(self, destination);
                let ms =self.max_speed;
                self.step(st,ms, dt);
                if (self.pos - destination).mag() < 5.0 {
                    self.behaviour = Behaviour::Gather(destination, Pos { x: 0.1, y: 0.0 });
                }
            }
            Behaviour::Attack(loc) => {
                let ens = map.find_enemies(self.pos, self.team);
                if ens.len() != 0 {
                    let mut enemies: Vec<(usize, f64, Pos)> = Vec::new();
                    for e in ens {
                        let e = Drone::get_data_from_drone_list(e, &drone_list);
                        if e.2 != self.team {
                            enemies.push((e.0, (e.1 - self.pos).mag(), e.1));
                        }
                    }
                    if enemies.len() != 0 {
                        let closest = enemies
                            .iter()
                            .min_by(|e1, e2| e1.1.partial_cmp(&e2.1).unwrap())
                            .unwrap();
                        //self.behaviour = Behaviour::Persue(closest.0, closest.2, Box::new(self.behaviour));
                        let old_behafiour = ::std::mem::replace(
                            &mut self.behaviour,
                            Behaviour::Persue(closest.0, closest.2, Box::new(Behaviour::Idle)),
                        );
                        ::std::mem::replace(
                            &mut self.behaviour,
                            Behaviour::Persue(closest.0, closest.2, Box::new(old_behafiour)),
                        );
                    }
                }
                steering::walk(self, dt, loc);
            }
            Behaviour::Evade(id, prev_pos, ref beh_box) => {}
            Behaviour::Persue(id, prev_pos, ref beh_box) => {}
            Behaviour::Idle => (),
        }
    }

    pub fn set_destination(&mut self, destination: Pos) {
        self.behaviour = Behaviour::Move(destination);
    }

    fn get_data_from_drone_list(id: usize, drone_list: &Vec<(usize, Pos, u8)>) -> (usize, Pos, u8) {
        for d in drone_list {
            if id == d.0 {
                return *d;
            }
        }
        panic!("Drone id {} not in drone_list.", id);
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
    Gather(Pos, Pos),
    ReturnTb(Pos),
    ReturnGathering(Pos),
    Attack(Pos),
    Evade(usize, Pos, Box<Behaviour>),
    Persue(usize, Pos, Box<Behaviour>),
}
