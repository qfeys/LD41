extern crate console;

use drone::*;
use base::*;

#[allow(unused_must_use)]
pub fn print(resources: f64, prod_queue: &Vec<Order>) {
    let mut lines = 0;
    let term = console::Term::stdout();
    let width = term.size().0;
    let mut output = String::from("LD 41 rps");
    lines += 1;
    let s = format!("Food: {}", resources);
    output = add_line(output, &s, width);
    lines += 1;
    let mut s = String::from("Production:");
    for order in prod_queue {
        match order.unit {
            unit_type::Worker { cargo: _ } => s.push_str(" W"),
            unit_type::Soldier => s.push_str(&" S"),
        }
    }
    output = add_line(output, &s, width);
    lines += 1;
    output = add_line(
        output,
        (String::from("Next unit: ")
            + progress_bar(
                match prod_queue.get(0) {
                    Some(o) => o.time_left,
                    None => 0.0,
                },
                4.0,
            ).as_str())
            .as_str(),
        width,
    );
    lines += 1;
    term.write_line(output.as_str());
    term.move_cursor_up(lines);
}

fn add_line(origin: String, new: &str, len: u16) -> String {
    let space = len - new.len() as u16;
    let mut spaces = String::from("");
    for _i in 0..space {
        spaces.push_str(" ");
    }
    origin + "\n" + new + spaces.as_str()
}

fn progress_bar(progress: f64, car_per_unit: f64) -> String {
    let carets = (progress * car_per_unit) as u32;
    let mut st = String::from("");
    for _i in 0..carets {
        st.push('#');
    }
    st
}
