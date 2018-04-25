use Pos;

#[derive(Debug)]
pub struct GameStateData {
    pub resources_players: Vec<f64>,
    pub base_locations: Vec<Pos>,
    pub debug_line: String,
}

impl GameStateData {
    pub fn new(base_locations: Vec<Pos>) -> GameStateData {
        if base_locations.len() != ::NUM_OF_PLAYERS {
            panic!(
                "You gave {} base locations but {} teams.",
                base_locations.len(),
                ::NUM_OF_PLAYERS
            );
        }
        GameStateData {
            resources_players: vec![1.0; ::NUM_OF_PLAYERS],
            base_locations,
            debug_line: String::from(""),
        }
    }

    pub fn deposite_resource(&mut self, amount: f64, team: u8) {
        self.resources_players[team as usize] += amount;
    }

    pub fn allocate_resource(&mut self, amount: f64, team: u8) -> bool {
        if self.resources_players[team as usize] >= amount {
            self.resources_players[team as usize] -= amount;
            return true;
        }
        false
    }
}
