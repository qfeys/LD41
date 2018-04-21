use drone::*;
use base::*;

#[derive(Debug)]
pub enum Entity {
    drone(Drone),
    base(Base),
}
