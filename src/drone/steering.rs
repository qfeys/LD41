use Pos;
use Drone;

const MASS: f64 = 1.0;

// Returns the steering vector
pub fn seek(drone: &Drone, target: Pos) -> Pos {
    let desired_velocity = (target - drone.pos).norm(drone.max_speed);
    let steering = desired_velocity - drone.vel;
    steering
}

impl Drone {
    pub fn step(&mut self, steering: Pos, dt: f64) {
        let acc = self.max_force / MASS;
        let mut dv = steering.norm(acc) * dt;
        // Decompose delta v vector
        let fwd = self.vel.norm(1.0) * (dv * self.vel.norm(1.0));
        let side = dv - fwd;
        if fwd * self.vel < 0.0 {
            dv = fwd * 2.0 + side;
        }
        self.vel = self.vel + dv;
        if self.vel.mag() > self.max_speed {
            self.vel = self.vel.norm(self.max_speed);
        }
        let dp = self.vel * dt;
        self.pos = self.pos + dp;
    }
}

pub fn walk(drone: &mut Drone, dt: f64, destination: Pos) {
    use std::f64::consts::PI;
    let mut rot = f64::atan2(drone.vel.y, drone.vel.x);
    let pi2 = PI * 2.0;
    let dir = destination - drone.pos;
    let angle = f64::atan2(dir.y, dir.x);
    let diff = (((angle - rot) % pi2) + pi2) % pi2;
    let max_turn = dt * 2.0;
    if diff < PI {
        if diff > PI * 2.0 - max_turn {
            rot += diff;
        } else {
            rot += max_turn;
        }
    } else {
        if diff < max_turn {
            rot -= diff;
        } else {
            rot -= max_turn
        }
    }
    rot = ((rot % pi2) + pi2) % pi2;
    drone.pos.x += drone.max_speed * dt * rot.cos();
    drone.pos.y += drone.max_speed * dt * rot.sin();
}
