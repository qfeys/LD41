const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const CYAN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
const MAGENTA: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
const PURPLE: [f32; 4] = [0.0, 0.5, 1.0, 1.0];
const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

pub fn team(team: u8) -> [f32; 4] {
    match team % 8 {
        0 => BLUE,
        1 => RED,
        2 => GREEN,
        3 => YELLOW,
        4 => CYAN,
        5 => MAGENTA,
        6 => ORANGE,
        7 => PURPLE,
        _ => panic!("wtf remainder 8??"),
    }
}

pub fn accent(team: u8) -> [f32; 4] {
    match team % 8 {
        0 => RED,
        1 => BLUE,
        2 => RED,
        3 => BLUE,
        4 => RED,
        5 => GRAY,
        6 => BLUE,
        7 => GRAY,
        _ => panic!("wtf remainder 8??"),
    }
}

pub fn soft(color: [f32; 4]) -> [f32; 4] {
    [color[0], color[1], color[2], 0.75]
}
