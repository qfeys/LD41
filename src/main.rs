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

pub mod drone;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [1200, 800])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        drones: Vec::new(),
    };
    app.drones.push(Drone::new());

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    drones: Vec<Drone>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let drones = &self.drones;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for d in drones {
                d.draw(gl, &c, args.width, args.height, 0.0, 0.0, 1.0);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        for d in &mut self.drones {
            d.walk(args.dt);
            //println!("{:?} + {}", d, args.dt);
        }
    }
}

#[derive(Debug)]
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
