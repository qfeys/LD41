extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::*;

use drone::*;
use base::*;
use gui::*;
use map::*;

pub mod drone;
pub mod base;
pub mod gui;
pub mod map;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

#[allow(unused_mut)]

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("LD 41", [WIDTH, HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        drones: Vec::new(),
        bases: Vec::new(),
        gui: Gui::new(),
        map: Map::new(),
    };
    app.drones.push(Drone::new());
    app.bases.push(Base::new());

    let mut events = Events::new(EventSettings::new());
    let mut x_center = 0.0;
    let mut y_center = 0.0;
    let mut scale = 1.0;
    //let mut mouse_s_pos: (f64, f64) = (0.0, 0.0);
    let mut mouse_w_pos: Pos = Pos { x: 0.0, y: 0.0 };
    let mut last_mouse_pos = mouse_w_pos;
    while let Some(e) = events.next(&mut window) {
        // RENDERING
        if let Some(r) = e.render_args() {
            app.render(&r, x_center, y_center, scale);
        }
        // MOUSE POSITION
        if let Some(pos) = e.mouse_cursor_args() {
            let mouse_s_pos = (pos[0], pos[1]);
            mouse_w_pos = Pos {
                x: (mouse_s_pos.0 - WIDTH as f64 / 2.0) / scale + x_center,
                y: (mouse_s_pos.1 - HEIGHT as f64 / 2.0) / scale + y_center,
            };
            app.gui.set_latest_mouse_pos(mouse_w_pos);
        }
        // BUTTONS
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Left) => {
                    x_center += -5.0;
                }
                Button::Keyboard(Key::Right) => {
                    x_center += 5.0;
                }
                Button::Keyboard(Key::Up) => {
                    y_center += -5.0;
                }
                Button::Keyboard(Key::Down) => {
                    y_center += 5.0;
                }
                Button::Mouse(MouseButton::Left) => {
                    last_mouse_pos = mouse_w_pos;
                    app.gui.start_box_draw(mouse_w_pos);
                }
                Button::Mouse(MouseButton::Right) => {
                    app.set_destination_selection(mouse_w_pos);
                }
                Button::Keyboard(Key::A) => {
                    app.bases[0].queue_worker();
                }
                Button::Keyboard(Key::S) => {
                    app.bases[0].queue_soldier();
                }
                _ => (),
            }
        };
        // BUTTONS RELEASE
        if let Some(button) = e.release_args() {
            if let Button::Mouse(MouseButton::Left) = button {
                let new_mouse_pos = mouse_w_pos;
                app.gui.end_box_draw();
                app.get_all_drones(last_mouse_pos, new_mouse_pos);
            }
        }
        // UPDATE
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

pub struct App<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    drones: Vec<Drone>,
    bases: Vec<Base>,
    gui: Gui,
    map: Map<'a>,
}

impl<'a> App<'a> {
    fn render(&mut self, args: &RenderArgs, x_center: f64, y_center: f64, scale: f64) {
        const GREEN: [f32; 4] = [0.1, 0.2, 0.1, 1.0];

        let drones = &self.drones;
        let bases = &self.bases;
        let gui = &mut self.gui;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for d in drones {
                d.draw(gl, &c, args.width, args.height, x_center, y_center, scale)
            }
            for b in bases {
                b.draw(gl, &c, args.width, args.height, x_center, y_center, scale)
            }

            gui.render(args, gl, &c, x_center, y_center, scale);
        });
    }

    fn update<'a>(&mut'a self, args: &UpdateArgs) {
        for mut d in &mut self.drones {
            d.walk(args.dt);
        }

        self.map.sort_drones(&self.drones);
        for mut b in &mut self.bases {
            let new_drone = b.update(args.dt);
            if let Some(nd) = new_drone {
                let d = Drone::from_pos_n_type(b.pos, nd);
                self.drones.push(d);
                println!("Drone spawned",);
            }
        }

        //println!("{:?} + {}", d, args.dt);
    }

    fn get_all_drones(&mut self, corner1: Pos, corner2: Pos) -> Vec<&Drone> {
        let min_corner = Pos {
            x: f64::min(corner1.x, corner2.x),
            y: f64::min(corner1.y, corner2.y),
        };
        let max_corner = Pos {
            x: f64::max(corner1.x, corner2.x),
            y: f64::max(corner1.y, corner2.y),
        };
        let mut selection: Vec<&Drone> = Vec::new();
        for mut d in &mut self.drones {
            if d.pos.x > min_corner.x && d.pos.x < max_corner.x && d.pos.y > min_corner.y
                && d.pos.y < max_corner.y
            {
                d.is_selected = true;
                selection.push(d);
            } else {
                d.is_selected = false;
            }
        }
        selection
    }

    fn set_destination_selection(&mut self, destination: Pos) {
        for mut d in &mut self.drones {
            if d.is_selected {
                d.set_destination(destination);
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    // Screen coordinates
    pub fn s_cor(
        &self,
        base_context: &Context,
        s_width: u32,
        s_height: u32,
        x_center: f64,
        y_center: f64,
        scale: f64,
    ) -> [[f64; 3]; 2] {
        base_context
            .transform
            .trans(s_width as f64 / 2.0, s_height as f64 / 2.0)
            .trans((self.x - x_center) / scale, (self.y - y_center) / scale)
    }

    pub fn mag(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn norm(self, length: f64) -> Pos {
        let factor = length / self.mag();
        Pos {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

use std::ops::Sub;

impl Sub for Pos {
    type Output = Pos;
    fn sub(self, other: Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
