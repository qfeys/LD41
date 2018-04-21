use drone::*;
use base::*;
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Entity {
    drone(Drone),
    base(Base),
}
