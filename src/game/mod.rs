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

/// Holds the data to display an atom on the window
struct AtomShape<'a> {
    circle: CircleShape<'a>,
    symbol: Text<'a>
}

/// Holds the shapes of the atoms on the board, to avoid regenerating them each
/// time the window is re-drawn, and the state of the board
pub struct Board<'a> {
    atoms: Vec<AtomShape<'a>>,
    state: GameState
}


const CIRCLE_RADIUS: f32 = 100.0;
const CIRCLE_XC: f32 = super::HEIGHT / 2.0;
const CIRCLE_YC: f32 = super::WIDTH  / 2.0;

const ATOM_RADIUS: f32 = 15.0;
const POINT_COUNT: u32 = 30;

// pub fn draw_board(window: &mut RenderWindow, board: &Board, font: &Font) {

// }

fn nth_atom_coord(i: usize, n: usize) -> (f32, f32) {
    if n == 0 {
        return (CIRCLE_XC, CIRCLE_YC - CIRCLE_RADIUS);
    }
    let theta = (i as f32) * 2. * PI / (n as f32);
    let dx = CIRCLE_RADIUS as f32 * theta.sin();
    let dy = CIRCLE_RADIUS as f32 * theta.cos();

    (CIRCLE_XC + dx, CIRCLE_YC - dy)
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

impl<'a> AtomShape<'a> {
    fn from_atom(atom: &Atom, font: &'a Font) -> Self {
        let (color, text) = match *atom {
            Atom::Plus => {     (Color::RED,   "+") },
            Atom::Minus => {    (Color::BLUE,  "-") }
            Atom::DarkPlus => { (Color::BLACK, "+") }
            Atom::Atom(z) => {
                (zth_color(z), ATOMS_SYMBOLS[z as usize])
            },
            Atom::None => { panic!("uninitialized atom")}
        };
        let mut circle_shape = CircleShape::new(ATOM_RADIUS, POINT_COUNT);
        circle_shape.set_fill_color(color);
        circle_shape.fill_color();

        let mut text_shape = Text::new(text, font, 12);
        text_shape.set_fill_color(Color::BLACK);

        Self {
            circle: circle_shape,
            symbol: text_shape
        }
    }

    fn set_position(&mut self, pos: (f32, f32)) {
        let (x, y) = pos;
        self.circle.set_position((x - ATOM_RADIUS, y - ATOM_RADIUS));

        let rect = self.symbol.global_bounds();
        self.symbol.set_position((x - rect.width / 2.0, y - rect.height / 2.0));
    }

    fn draw_on(&self, window: &mut RenderWindow) {
        window.draw(&self.circle);
        window.draw(&self.symbol);
    }
}

impl<'a> Board<'a> {
    pub fn new(font: &'a Font) -> Self {
        let state = GameState::start_game();
        let mut atoms = Vec::new();

        let n = state.atoms.len();
        for i in 0..n {
            let mut shape = AtomShape::from_atom(&state.atoms[i], font);
            shape.set_position(nth_atom_coord(i, n));
            atoms.push(shape);
        }
        let mut shape = AtomShape::from_atom(&state.incoming, font);
        shape.set_position((CIRCLE_XC, CIRCLE_YC));
        atoms.push(shape);

        Self {
            atoms: atoms,
            state: state
        }
    }

    pub fn draw_on(&self, window: &mut RenderWindow) {
        for atom in &self.atoms {
            atom.draw_on(window);
        }
    }
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