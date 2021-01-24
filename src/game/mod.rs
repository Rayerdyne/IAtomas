mod state;
mod atoms_data;

pub use atoms_data::*;
pub use state::*;

use std::f32::consts::PI;
use std::num::ParseIntError;

use sfml::{
    graphics::{CircleShape, Color, Font, Rect, RenderTarget, RenderWindow, 
    Shape, Text, Transformable},
};


const CIRCLE_RADIUS: f32 = 100.0;
const ATOM_RADIUS: f32 = 15.0;
const POINT_COUNT: u32 = 30;

pub fn draw_state(window: &mut RenderWindow, state: &GameState, font: &Font) {
    let n = state.atoms.len();

    for i in 0..n {
        let (color, text) = match state.atoms[i] {
            Atom::Plus => {     (Color::RED,   "+") },
            Atom::Minus => {    (Color::BLUE,  "-") }
            Atom::DarkPlus => { (Color::BLACK, "+") }
            Atom::Atom(z) => {
                (zth_color(z), ATOMS_SYMBOLS[z as usize])
            },
        };
        let (x, y) = nth_atom_coord(i, n);
        println!("i: {}, x: {}, y: {}, color: {:?}", i, x, y, color);
        let mut shape = CircleShape::new(ATOM_RADIUS, POINT_COUNT);
        shape.set_position((x - ATOM_RADIUS, y - ATOM_RADIUS));
        shape.set_fill_color(color);
        shape.fill_color();

        let mut text_shape = Text::new(text, font, 12);
        let rect = text_shape.global_bounds();
        text_shape.set_position((x - rect.width / 2.0, y - rect.height / 2.0));
        text_shape.set_fill_color(Color::BLACK);

        window.draw(&shape);
        window.draw(&text_shape);
    }

}

fn nth_atom_coord(i: usize, n: usize) -> (f32, f32) {
    let (xc, yc) = (super::WIDTH / 2.0, super::HEIGHT / 2.0);
    if n == 0 {
        return (xc, yc - CIRCLE_RADIUS);
    }
    let theta = (i as f32) * 2. * PI / (n as f32);
    let dx = CIRCLE_RADIUS as f32 * theta.sin();
    let dy = CIRCLE_RADIUS as f32 * theta.cos();

    (xc + dx, yc - dy)
}

fn zth_color(z: u8) -> Color {
    let s = ATOMS_COLORS[z as usize];
    let (r, g, b) = color_from_hex(s).expect("Unable to parse color !?");
    Color::rgb(r, g, b)
}

fn color_from_hex(s: &str) -> Result <(u8, u8, u8), ParseIntError> {
    let without_prefix = s.trim_start_matches("#");
    let r = u8::from_str_radix(&without_prefix[0..2], 16)?;
    let g = u8::from_str_radix(&without_prefix[2..4], 16)?;
    let b = u8::from_str_radix(&without_prefix[4..6], 16)?;

    Ok((r, g, b))
}

// extern crate lazy_static;

// use std::sync::Mutex;

// use lazy_static::lazy_static;


// struct StatePointer<'a> {
//     state: Option<&'a GameState>,
//     x: f64
// }

// lazy_static!{
//     static ref STATE_POINTER: Mutex<StatePointer<'static>> 
//         = Mutex::new(StatePointer::new());
// }

// fn function() -> f64 {
//     STATE_POINTER.lock().unwrap().add_x(1.0);
//     STATE_POINTER.lock().unwrap().get_x()
// }

// impl StatePointer<'_> {
//     fn new() -> Self {
//         StatePointer {
//             state: None,
//             x: 100.0
//         }
//     }

//     fn get_x(&self) -> f64 {
//         self.x
//     }

//     fn add_x(&mut self, y: f64) {
//         self.x = self.x + y;
//     }
// }