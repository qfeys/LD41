use std::collections::HashMap;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::*;
use super::*;
extern crate hprof;

const CHUNK_WIDTH: f64 = 100.0;

#[derive(Debug)]
pub struct Map {
    data: HashMap<Chunk, ChunkData>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Chunk {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ChunkData {
    growth_progress: f64,
    inhabitants: Vec<usize>,
    has_workers_t1: bool,
    has_workers_t2: bool,
    has_soldiers_t1: bool,
    has_soldiers_t2: bool,
    north: Option<Chunk>,
    east: Option<Chunk>,
    south: Option<Chunk>,
    west: Option<Chunk>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            data: HashMap::new(),
        }
    }

    pub fn sort_drones(&mut self, drones: &Vec<Drone>) {
        for ch_dat in self.data.values_mut() {
            ch_dat.inhabitants = Vec::new();
        }
        for d in drones {
            let x: i32 = (d.pos.x / CHUNK_WIDTH).round() as i32;
            let y: i32 = (d.pos.y / CHUNK_WIDTH).round() as i32;
            let chunk_found = self.data.contains_key(&Chunk { x, y });
            let chunk;
            if chunk_found == false {
                chunk = Chunk { x, y };
                self.data.insert(chunk, ChunkData::default());
            //println!("New Chunk at :{}, {}", x, y);
            } else {
                chunk = Chunk { x, y };
            }
            let ch_dat = self.data.entry(chunk).or_insert(ChunkData::default());
            match d.u_type {
                ::drone::unit_type::Worker { cargo: _ } => if d.team == 1 {
                    ch_dat.has_workers_t1 = true;
                } else {
                    ch_dat.has_workers_t2 = true;
                },
                ::drone::unit_type::Soldier => if d.team == 1 {
                    ch_dat.has_soldiers_t1 = true;
                } else {
                    ch_dat.has_soldiers_t2 = true;
                },
            };
            ch_dat.inhabitants.push(d.id);
        }
    }

    pub fn update_growth(&mut self, dt: f64, drone_list: &Vec<Drone>) {
        use std::f64::consts::PI;
        for ch_d in self.data.values_mut() {
            let mut worker_count = 0;
            let mut soldeier_count = 0;
            let mut i = 0;
            let mut total = ch_d.inhabitants.len();
            for dr in drone_list {
                if i == total {
                    break;
                }
                if dr.id == ch_d.inhabitants[i] {
                    match dr.u_type {
                        unit_type::Worker { cargo: _ } => worker_count += 1,
                        unit_type::Soldier => soldeier_count += 1,
                    }
                    i += 1;
                }
            }
            // Workers effort
            ch_d.growth_progress +=
                f64::atan(worker_count as f64 / 6.0) * 2.0 / PI * (1.0 - ch_d.growth_progress) * dt;
            // Base recovery
            if ch_d.growth_progress < 0.1 {
                ch_d.growth_progress += 0.005 * dt;
            } else if ch_d.growth_progress < 0.2 {
                ch_d.growth_progress += 0.0025 * dt;
            }
            // Base decay
            if ch_d.growth_progress > 0.5 {
                ch_d.growth_progress -= 0.01 * dt;
            }
            // Soldiers destruction
            ch_d.growth_progress -= 0.01 * soldeier_count as f64 * dt;
            if ch_d.growth_progress < 0.0 {
                ch_d.growth_progress = 0.0;
            }
        }
    }

    pub fn gather_resource(&mut self, pos: Pos, rate: f64, dt: f64) -> f64 {
        let x: i32 = (pos.x / CHUNK_WIDTH).round() as i32;
        let y: i32 = (pos.y / CHUNK_WIDTH).round() as i32;
        match self.data.get_mut(&Chunk { x, y }) {
            Some(v) => {
                //let a:()=*v;
                let rsrc = v.growth_progress * rate * dt;
                v.growth_progress -= rsrc;
                rsrc
            }
            None => panic!("invalid chnunk at x:{}, y:{}", x, y),
        }
    }

    pub fn render(
        &mut self,
        args: &RenderArgs,
        gl: &mut GlGraphics,
        c: &Context,
        x_center: f64,
        y_center: f64,
        scale: f64,
    ) {
        for (ch, ch_d) in &self.data {
            let rect: types::Rectangle<f64> = [
                ((ch.x as f64 - 0.5) * CHUNK_WIDTH - x_center) / scale + args.width as f64 / 2.0,
                ((ch.y as f64 - 0.5) * CHUNK_WIDTH - y_center) / scale + args.height as f64 / 2.0,
                CHUNK_WIDTH,
                CHUNK_WIDTH,
            ];
            let color: [f32; 4] = [0.1, ch_d.growth_progress as f32, 0.1, 1.0];
            Rectangle::new(color).draw(rect, &c.draw_state, c.transform, gl)
        }
    }
}

impl ChunkData {
    pub fn default() -> ChunkData {
        ChunkData {
            growth_progress: 0.2,
            inhabitants: Vec::new(),
            has_workers_t1: false,
            has_workers_t2: false,
            has_soldiers_t1: false,
            has_soldiers_t2: false,
            north: None,
            east: None,
            south: None,
            west: None,
        }
    }
}
