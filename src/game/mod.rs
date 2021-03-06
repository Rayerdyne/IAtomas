mod state;
mod atoms_data;

pub use atoms_data::*;
pub use state::*;

use std::f32::consts::PI;
use std::num::ParseIntError;

use sfml::{
    graphics::{CircleShape, Color, Font, RenderTarget, RenderWindow, 
    Shape, Text, Transformable},
};

/// Holds the data to display an atom on the window
#[derive(Clone, Debug)]
pub struct AtomShape<'a> {
    circle: CircleShape<'a>,
    symbol: Text<'a>
}

/// Holds the shapes of the atoms on the board, to avoid regenerating them each
/// time the window is re-drawn, and the state of the board
#[derive(Clone, Debug)]
pub struct Board<'a, 'b> {
    state: GameState<'a>,
    minused: bool,
    best_val: u8,
    best_text: Text<'b>,
    score_text: Text<'b>,
    font: &'b Font,
}


const CIRCLE_RADIUS: f32 = 100.0;
const CIRCLE_XC: f32 = super::HEIGHT / 2.0;
const CIRCLE_YC: f32 = super::WIDTH  / 2.0;

const ATOM_RADIUS: f32 = 15.0;
const POINT_COUNT: u32 = 30;

const BEST_X: f32 = CIRCLE_XC;
const BEST_Y: f32 = 20.0;
const SCORE_X: f32 = 10.0;
const SCORE_Y: f32 = 20.0;

fn nth_atom_coord(i: usize, n: usize) -> (f32, f32) {
    if n == 0 {
        return (CIRCLE_XC, CIRCLE_YC - CIRCLE_RADIUS);
    }
    let theta = (i as f32) * 2. * PI / (n as f32);
    let dx = CIRCLE_RADIUS as f32 * theta.sin();
    let dy = CIRCLE_RADIUS as f32 * theta.cos();

    (CIRCLE_XC + dx, CIRCLE_YC - dy)
}

/// Retreives the color associated to the atom of atomic number `z`
fn zth_color(z: u8) -> Color {
    let s = ATOMS_COLORS[z as usize];
    let (r, g, b) = color_from_hex(s).expect("Unable to parse color !?");
    Color::rgb(r, g, b)
}

/// Builds a color RGB triplet from a formatted string.
fn color_from_hex(s: &str) -> Result <(u8, u8, u8), ParseIntError> {
    let without_prefix = s.trim_start_matches("#");
    let r = u8::from_str_radix(&without_prefix[0..2], 16)?;
    let g = u8::from_str_radix(&without_prefix[2..4], 16)?;
    let b = u8::from_str_radix(&without_prefix[4..6], 16)?;

    Ok((r, g, b))
}

/// Retrieves the color and the text (symbol if regular atom, or `+`, `-`, ...)
/// from an `AtomType`
fn atom_color_text<'a>(t: &AtomType) -> (Color, &'static str) {
    match *t {
        AtomType::Plus => {     (Color::RED,   "+") },
        AtomType::Minus => {    (Color::BLUE,  "-") }
        AtomType::DarkPlus => { (Color::BLACK, "+") }
        AtomType::Neutrino => { (Color::WHITE, " ") }
        AtomType::Atom(z) => {
            (zth_color(z), ATOMS_SYMBOLS[z as usize])
        },
        AtomType::None => { panic!("uninitialized atom")}
    }
}

impl<'a> AtomShape<'a> {
    /// Constructs an atom shape from a given `AtomType`
    fn from_atom_type(atom_type: &AtomType, font: &'a Font) -> Self {
        let (color, text) = atom_color_text(atom_type);
        // println!("{:?} -> {}", atom_type, text);
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

    // fn change_to(&mut self, atom_type: &AtomType) {
    //     let (color, text) = atom_color_text(atom_type);
    //     self.circle.set_fill_color(color);
    //     self.symbol.set_string(text);
    // }

    /// Set the position of the shape, that is, updates the coordinates of the
    /// atom's circle and symbol
    fn set_position(&mut self, pos: (f32, f32)) {
        let (x, y) = pos;
        self.circle.set_position((x - ATOM_RADIUS, y - ATOM_RADIUS));

        let rect = self.symbol.global_bounds();
        let pos = (x - rect.width / 2.0, y - rect.height / 2.0);
        // println!("{:?} vs {:?}", pos, (x, y));
        self.symbol.set_position(pos);
    }

    /// Draw this shape on a given `RenderWindow`
    fn draw_on(&self, window: &mut RenderWindow) {
        window.draw(&self.circle);
        window.draw(&self.symbol);
        
        // println!("write: {} @\t {:?}\t {:?}", self.symbol.string().to_rust_string(),
        //                               self.symbol.position(),
        //                               self.symbol.fill_color());
    }
}

impl<'a, 'b: 'a> Board<'a, 'b> {
    /// Create a new `Board` with given `GameState`, no shape built
    pub fn from_state(state: GameState<'a>, font: &'b Font) -> Self {
        let mut b_text = Text::new(ATOMS_NAMES[0], font, 20);
        b_text.set_fill_color(Color::YELLOW);
        b_text.set_outline_thickness(0.3);
        let rect = b_text.global_bounds();
        b_text.set_position((BEST_X - rect.width / 2.0, BEST_Y));

        let mut s_text = Text::new("0", font, 20);
        s_text.set_fill_color(Color::YELLOW);
        s_text.set_outline_thickness(0.3);
        let rect = s_text.global_bounds();
        s_text.set_position((SCORE_X - rect.width / 2.0, SCORE_Y));
        Self {
            state: state,
            minused: false,
            best_text: b_text,
            best_val: 0,
            score_text: s_text,
            font: font
        }
    }

    /// Create a new `Board` with default starting board
    pub fn new(font: &'b Font) -> Self {
        Self::from_state(GameState::start_game(), font)
    }

    /// Updates all the shapes, built the ones that are not yet built
    pub fn update_shapes(&mut self) {
        let n = self.state.atoms.len();
        for i in 0..n {
            let j = i + self.state.shift % n;
            let a = &mut self.state.atoms[j];
            match a.shape.as_mut() {
                Some(shape) => shape.set_position(nth_atom_coord(j, n)),
                None => {
                    let mut new_shape = AtomShape::from_atom_type(&a.t, 
                                                                   self.font);
                    new_shape.set_position(nth_atom_coord(j, n));
                    a.shape = Some(new_shape);
                }
            }
        }

        let mut shape = AtomShape::from_atom_type(&self.state.incoming.t, 
                                                   self.font);
        shape.set_position((CIRCLE_XC, CIRCLE_YC));
        self.state.incoming.shape = Some(shape);
    }


    // /// Regenrates the shapes, as I'm lazy
    // pub fn regen(&mut self) {
    //     self.atoms.clear();
        
    //     let n = self.state.atoms.len();
    //     for i in 0..n {
    //         let j = i + self.state.shift % n;
    //         let mut shape = AtomShape::from_atom_type(&self.state.atoms[j].t,
    //                                               self.font);
    //         shape.set_position(nth_atom_coord(j, n));
    //         self.atoms.push(shape);
    //     }
    //     let mut shape = AtomShape::from_atom_type(&self.state.incoming.t, 
    //                                                self.font);
    //     shape.set_position((CIRCLE_XC, CIRCLE_YC));
    //     self.atoms.push(shape);

    // }

    /// Draws all the atoms on `window`
    pub fn draw_on(&self, window: &mut RenderWindow) {
        for atom in &self.state.atoms {
            if let Some(shape) = &atom.shape {
                shape.draw_on(window);
            }
        }
        if let Some(shape) = &self.state.incoming.shape {
            shape.draw_on(window);
        }
        window.draw(&self.best_text);
        window.draw(&self.score_text);
    }

    /// Reacts to a click event in `x0`, `y0`.
    pub fn click(&mut self, x0: i32, y0: i32) {
        let (x, y) = (x0 as f32, y0 as f32);
        let (dx, dy) = (x - CIRCLE_XC, y - CIRCLE_YC);

        let d_squared = dx.powi(2) + dy.powi(2);

        if d_squared < ATOM_RADIUS.powi(2) && self.minused {
            self.state.incoming = Atom::from_type_with_shape(AtomType::Plus,
                                            self.font, (CIRCLE_XC, CIRCLE_YC));
            self.minused = false;
        }
        else if  d_squared < (CIRCLE_RADIUS + ATOM_RADIUS).powi(2) {
            if self.state.incoming.t == AtomType::Minus {
                self.pick_atom(dx, dy, false);
            }
            else if self.state.incoming.t == AtomType::Neutrino {
                self.pick_atom(dx, dy, true);
            }
            else {
                self.shot_atom(dx, dy);
            }
        } 
    }

    /// Shots the incoming atom, where `dx`, `dy` are the relative distance
    /// to the center of the circle.
    fn shot_atom(&mut self, dx: f32, dy: f32) {
        let theta = Board::angle(dx, dy);
        let n = self.state.atoms.len();
        
        // theta += 360.0 / n as f32;
        let i = theta * n as f32 / 360.0;
        let j = if n == 0 { 0 } else 
            { ((i.floor() as usize) + n - self.state.shift + 1) % n };

        let max = self.state.play(j as u8);
        if max > self.best_val {
            self.best_val = max;
            self.best_text.set_string(ATOMS_NAMES[max as usize]);
            let rect = self.best_text.global_bounds();
            self.best_text.set_position((BEST_X - rect.width / 2.0, BEST_Y));
        }
        self.score_text.set_string(&format!("{}", self.state.score));
        self.update_shapes();
    }

    /// When the current incoming atom is either a Minus or a Neutrino, reacts
    /// user click at `dx, dy` (measured relatively to the center of the 
    /// circle). The additional parameter `is_neutrino` will be raised if the 
    /// incoming atom is a Neutrino.
    fn pick_atom(&mut self, dx: f32, dy: f32, is_neutrino: bool) {
        if dx.powi(2) + dy.powi(2) > (CIRCLE_RADIUS - ATOM_RADIUS).powi(2) {
            let n = self.state.atoms.len();
            let theta = Board::angle(dx, dy) + 360.0 / (2.0 * n as f32);

            let i = theta * n as f32 / 360.0;
            // let j = (i.floor() as usize) % n;
            let j = ((i.floor() as usize)) % n;
            self.state.incoming = Atom::from_type_with_shape(
                self.state.atoms[j].t.clone(),
                self.font,
                (CIRCLE_XC, CIRCLE_YC));
            if !is_neutrino {
                self.state.atoms.remove(j);
                self.minused = true;
            }
        }
        self.state.update_plus();
        self.update_shapes();
    }

    /// Computes the angle (centered in `(0, 0)`) associated to the coordinates
    /// `(dy, dy)`.
    fn angle(dx: f32, dy: f32) -> f32 {
        let mut theta = (-dx / dy).atan();
        theta = theta * 360.0 / (2.0 * PI);
        if dy > 0.0 {
            theta = 180.0 + theta;
        } else if dx < 0.0 && dy < 0.0 {
            theta = 360.0 + theta;
        }

        theta
    }

    #[allow(dead_code)]
    /// Prints the info about the state in this `Board`
    pub fn info(&self) {
        self.state.info();
    }

    /// Force the incoming atom in the state to a given type.
    ///
    /// Obviously, for testing purposes.
    pub fn set_state_incoming(&mut self, atom_type: AtomType) {
        self.state.incoming = Atom::from_type(atom_type);
        self.update_shapes();
    }
}