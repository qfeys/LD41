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

    pub fn deposite_resource(&mut self, amount: f64, team: u8){
    	if team == 1{
    		self.resources_player_1 += amount;
    	}else if team == 2{
    		self.resources_player_2 += amount;
    	}else{
    		panic!("Invalid team. You tried giving team {:?} resources.", team);
    	}
    }

    pub fn allocate_resource(&mut self, amount: f64, team: u8) -> bool{
    	if team == 1{
    		if self.resources_player_1 >= amount{
    		self.resources_player_1 -= amount;
    		return true;}
    	}else if team == 2{
    		if self.resources_player_2 >= amount{
    		self.resources_player_2 -= amount;
    		return true;}
    	}else{
    		panic!("Invalid team. You tried giving team {:?} resources.", team);
    	}
    	false
    }
}
