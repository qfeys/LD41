use gsd::*;
use drone::*;
use base::*;

#[derive(Debug)]
pub struct Aicontr {
    teamid: usize,
    workers: Vec<usize>,
    soldiers: Vec<usize>,
}

impl Aicontr {
    pub fn new(teamid: usize) -> Aicontr {
        Aicontr {
            teamid,
            workers: Vec::new(),
            soldiers: Vec::new(),
        }
    }

    pub fn step(&mut self, gsd: &mut GameStateData, drones: &Vec<Drone>, bases: &mut Vec<Base>) {
        let resources = gsd.resources_players[self.teamid];
        let base = &mut bases[self.teamid];
        //println!("Team {} has {} resources.",self.teamid, resources );
        if resources >= 1.0 {
            base.queue_worker(gsd);
            println!("Team {} started a new worker.", self.teamid);
        }
        let last_drone_id = <usize>::max(
            *self.workers.last().unwrap_or(&0),
            *self.soldiers.last().unwrap_or(&0),
        );
        let last_drone_index = drones.binary_search_by(|d| d.id.cmp(&last_drone_id));
        if let Ok(ldi) = last_drone_index {
            for d in &drones[ldi + 1..] {
                if d.team == self.teamid as u8 {
                    match d.u_type {
                        unit_type::Worker { cargo: _ } => self.workers.push(d.id),
                        unit_type::Soldier => self.soldiers.push(d.id),
                    }
                    println!("Team {} added drone {}", self.teamid, d.id);
                }
            }
        }
    }
}
