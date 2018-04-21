use opengl_graphics::*;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use Pos;

#[derive(Debug)]
pub struct Gui {
    pub draw_selection_box: bool,
    selection_box_first_corner: Pos,
    selection_box_second_corner: Pos,
}

impl Gui {
    pub fn new() -> Gui {
        Gui {
            draw_selection_box: false,
            selection_box_first_corner: Pos { x: 0.0, y: 0.0 },
            selection_box_second_corner: Pos { x: 0.0, y: 0.0 },
        }
    }

    pub fn start_box_draw(&mut self, start_corner: Pos) {
        self.selection_box_first_corner = start_corner;
        self.draw_selection_box = true;
    }

    pub fn set_latest_mouse_pos(&mut self, mouse_w_pos: Pos) {
        if self.draw_selection_box == true {
            self.selection_box_second_corner = mouse_w_pos;
        }
    }

    pub fn end_box_draw(&mut self) {
        self.draw_selection_box = false;
    }

    pub fn render(
        &mut self,
        args: &RenderArgs,
        gl: &mut GlGraphics,
        c: &Context,
        x_center: f64,
        y_center: f64,
        scale: f64,
    ) {
        const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 0.5];
        const NULL: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        if self.draw_selection_box == true {
            let border = rectangle::Border {
                color: GRAY,
                radius: 1.5,
            };
            let rect: types::Rectangle<f64> = [
                (f64::min(
                    self.selection_box_first_corner.x,
                    self.selection_box_second_corner.x,
                ) - x_center) / scale + args.width as f64 / 2.0,
                (f64::min(
                    self.selection_box_first_corner.y,
                    self.selection_box_second_corner.y,
                ) - y_center) / scale + args.height as f64 / 2.0,
                f64::abs(self.selection_box_first_corner.x - self.selection_box_second_corner.x)
                    / scale,
                f64::abs(self.selection_box_first_corner.y - self.selection_box_second_corner.y)
                    / scale,
            ];
            Rectangle::new_border(GRAY, 1.5).draw(rect, &c.draw_state, c.transform, gl)
        }
    }
}
