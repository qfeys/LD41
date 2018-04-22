use std::collections::HashMap;
use drone::*;

const CHUNK_WIDTH: f64 = 100.0;

#[derive(Debug)]
pub struct Map<'a> {
    data: HashMap<Chunk, ChunkData<'a>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Chunk {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ChunkData<'a> {
    growth_progress: f64,
    inhabitants: Vec<&'a Drone>,
    has_workers_t1: bool,
    has_workers_t2: bool,
    has_soldiers_t1: bool,
    has_soldiers_t2: bool,
    North: Option<Chunk>,
    East: Option<Chunk>,
    South: Option<Chunk>,
    West: Option<Chunk>,
}

impl<'a> Map<'a> {
    pub fn new() -> Map<'a> {
        Map {
            data: HashMap::new(),
        }
    }

    pub fn sort_drones(&mut self, drones: &'a Vec<Drone>) {
        for d in drones {
            let x: i32 = (d.pos.x / CHUNK_WIDTH).round() as i32;
            let y: i32 = (d.pos.y / CHUNK_WIDTH).round() as i32;
            let chunk_found = self.data.contains_key(&Chunk { x, y });
            let chunk;
            if chunk_found == false {
                chunk = Chunk { x, y };
                self.data.insert(chunk, ChunkData::default());
                println!("New Chunk at :{}, {}", x, y);
            } else {
                chunk = Chunk { x, y };
            }
            let ch_dat = self.data.entry(chunk).or_insert(ChunkData::default());
            match d.u_type {
                ::drone::unit_type::Worker => if d.team == 1 {
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
            ch_dat.inhabitants.push(d);
        }
    }
}

impl<'a> ChunkData<'a> {
    pub fn default() -> ChunkData<'a> {
        ChunkData {
            growth_progress: 0.1,
            inhabitants: Vec::new(),
            has_workers_t1: false,
            has_workers_t2: false,
            has_soldiers_t1: false,
            has_soldiers_t2: false,
            North: None,
            East: None,
            South: None,
            West: None,
        }
    }
}
