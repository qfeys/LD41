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

use entity::*;
use drone::*;
use base::*;

pub mod entity;
pub mod drone;
pub mod base;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

#[allow(unused_mut)]

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [WIDTH, HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        entities: Vec::new(),
    };
    app.entities.push(Entity::drone(Drone::new()));
    app.entities.push(Entity::base(Base::new()));

    let mut events = Events::new(EventSettings::new());
    let mut x_center = 0.0;
    let mut y_center = 0.0;
    let mut scale = 1.0;
    let mut mouse_s_pos: (f64, f64) = (0.0, 0.0);
    let mut mouse_w_pos: Pos = Pos{x:0.0,y:0.0};
    let mut last_mouse_pos = mouse_w_pos;
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, x_center, y_center, scale);
        }
        if let Some(pos) = e.mouse_cursor_args() {
            mouse_s_pos = (pos[0], pos[1]);
            mouse_w_pos = Pos {
                        x: (mouse_s_pos.0 - WIDTH as f64/2.0) / scale + x_center,
                        y: (mouse_s_pos.1 - HEIGHT as f64/2.0) / scale + y_center,
                    };
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Left) => {
                    x_center += -5.0;
                },
                Button::Keyboard(Key::Right) => {
                	x_center += 5.0;
                },
                Button::Keyboard(Key::Up) => {
                	y_center += -5.0;
                },
                Button::Keyboard(Key::Down) => {
                	y_center += 5.0;
                },
                Button::Mouse(MouseButton::Left) => {
                    last_mouse_pos = mouse_w_pos;
                }
                _ => (),
            }
        };
        if let Some(button) = e.release_args(){
        	let new_mouse_pos = mouse_w_pos;
        	app.Get_all_drones(last_mouse_pos,new_mouse_pos);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    entities: Vec<Entity>,
}

impl App {
    fn render(&mut self, args: &RenderArgs, x_center: f64, y_center: f64, scale: f64) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let entities = &self.entities;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for e in entities {
                match *e {
                    Entity::drone(ref d) => {
                        d.draw(gl, &c, args.width, args.height, x_center, y_center, scale)
                    }
                    Entity::base(ref b) => {
                        b.draw(gl, &c, args.width, args.height, x_center, y_center, scale)
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        for e in &mut self.entities {
            match *e {
                Entity::drone(ref mut d) => d.walk(args.dt),
                Entity::base(_) => (),
            }

            //println!("{:?} + {}", d, args.dt);
        }
    }

    fn Get_all_drones(&self, corner1: Pos, corner2: Pos) -> Vec<Drone> {
        unimplemented!();
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
}
