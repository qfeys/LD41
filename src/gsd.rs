#[derive(Debug)]
pub struct GameStateData {
    pub resources_player_1: f64,
    pub resources_player_2: f64,
}

impl GameStateData {
    pub fn new() -> GameStateData {
        GameStateData {
            resources_player_1: 0.0,
            resources_player_2: 0.0,
        }
    }
}
