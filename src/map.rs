use std::collections::HashMap;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::*;
use super::*;
extern crate hprof;

const CHUNK_WIDTH: f64 = 100.0;
const BORDER_DIRS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

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
    has_workers: Vec<bool>,
    has_soldiers: Vec<bool>,
    borders: [Option<Chunk>; 8],
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
            ch_dat.has_workers = vec![false; ::NUM_OF_PLAYERS];
            ch_dat.has_soldiers = vec![false; ::NUM_OF_PLAYERS];
        }
        for d in drones {
            let chunk = Chunk::pos_to_chunk(d.pos);
            let chunk_found = self.data.contains_key(&chunk);
            let x = chunk.x;
            let y = chunk.y;
            if chunk_found == false {
                self.data.insert(chunk, ChunkData::default());
                for dir in 0..BORDER_DIRS.len() {
                    let test_chunk = Chunk {
                        x: x + BORDER_DIRS[dir].0,
                        y: y + BORDER_DIRS[dir].1,
                    };
                    if self.data.contains_key(&test_chunk) {
                        self.data.get_mut(&chunk).unwrap().borders[dir] = Some(test_chunk);
                        self.data.get_mut(&test_chunk).unwrap().borders[(dir + 4) % 8] =
                            Some(test_chunk);
                    }
                }
            }
            let ch_dat = self.data.entry(chunk).or_insert(ChunkData::default());
            match d.u_type {
                ::drone::unit_type::Worker { cargo: _ } => {
                    ch_dat.has_workers[d.team as usize] = true
                }

                ::drone::unit_type::Soldier => ch_dat.has_soldiers[d.team as usize] = true,
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

    // Returns id numbers of possible nearby enemies.
    // Being an enemy is not garanteed
    // Alies will definatly not always be given
    pub fn find_enemies(&self, pos: Pos, team: u8) -> Vec<usize> {
        let chunk = Chunk::pos_to_chunk(pos);
        let mut ret = Vec::new();
        if self.data[&chunk].contains_enemy_workers(team)
            || self.data[&chunk].contains_enemy_soldiers(team)
        {
            ret.extend_from_slice(&self.data[&chunk].inhabitants);
        }
        for dir in 0..BORDER_DIRS.len() {
            if let Some(ch) = self.data[&chunk].borders[dir] {
                if self.data[&ch].contains_enemy_workers(team)
                    || self.data[&ch].contains_enemy_soldiers(team)
                {
                    ret.extend_from_slice(&self.data[&ch].inhabitants);
                }
            }
        }
        ret
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

impl Chunk {
    pub fn pos_to_chunk(pos: Pos) -> Chunk {
        Chunk {
            x: (pos.x / CHUNK_WIDTH).round() as i32,
            y: (pos.y / CHUNK_WIDTH).round() as i32,
        }
    }
}

impl ChunkData {
    pub fn default() -> ChunkData {
        ChunkData {
            growth_progress: 0.2,
            inhabitants: Vec::new(),
            has_workers: vec![false; ::NUM_OF_PLAYERS],
            has_soldiers: vec![false; ::NUM_OF_PLAYERS],
            borders: [None; 8],
        }
    }

    pub fn contains_enemy_workers(&self, my_team: u8) -> bool {
        for team in 0..::NUM_OF_PLAYERS {
            if team == my_team as usize {
                continue;
            }
            if self.has_workers[team] {
                return true;
            }
        }
        false
    }

    pub fn contains_enemy_soldiers(&self, my_team: u8) -> bool {
        for team in 0..::NUM_OF_PLAYERS {
            if team == my_team as usize {
                continue;
            }
            if self.has_soldiers[team] {
                return true;
            }
        }
        false
    }
}
