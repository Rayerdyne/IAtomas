use std::{cmp::{Ordering, max}, usize};

use super::AtomShape;

use rand::Rng;
use rand_distr::{Bernoulli, Binomial, Distribution};
use sfml::graphics::Font;
use lazy_static::lazy_static;

lazy_static!{
    static ref BINOM: Binomial = Binomial::new(7, 0.5).unwrap();
    static ref BERN_02: Bernoulli = Bernoulli::new(0.2).unwrap();
}

/// Represents anything we can get when "drawing a new atom", that is, it takes
/// the `Atom`s, `Plus`, `Minus`, `DarkPlus` into account.
///
/// Antimatter will not be taken into account.
#[derive(Debug, Clone)]
pub struct Atom<'a> {
    pub t: AtomType, 
    pub shape: Option<AtomShape<'a>>
}

#[derive(Clone, Debug, Eq)]
pub enum AtomType {
    Atom(u8),
    DarkPlus,
    Minus,
    Neutrino,
    None,
    Plus, 
}

const PLUS_CHANCE: f64 = 0.23;
const MINUS_CHANCE: f64 = 0.05;
const DPLUS_CHANCE: f64 = 0.0125;
const WHITE_CHANCE: f64 = 1_f64 / 60_f64;

const C2: f64 = PLUS_CHANCE + MINUS_CHANCE;
const C3: f64 = C2 + DPLUS_CHANCE;
const C4: f64 = C3 + WHITE_CHANCE;

const MIN_DPLUS_SCORE: u32 = 750;
const MIN_WHITE_SCORE: u32 = 1500;


/// Represents the state of the game at some point.
///
/// Contains:
///
/// - `atoms`: a vector of `Atom`s
///
/// - `shift`: the shift, that is, as the vector of atoms is the least "word" 
/// of every possible rotation, as we consider two rotation of the same atoms 
/// to be the same, and symmetry doesn't matter neither
///
/// - `time`: the number of turns played since the start
///
/// - `incoming`: an option containing the incoming atom
///
/// - `score`: the score so far
///
/// - `pluses`: the indexes of the plus atoms on the board
#[derive(Debug, Clone)]
pub struct GameState<'a> {
    pub atoms: Vec<Atom<'a>>,
    pub shift: usize,
    pub time: u32,
    pub incoming: Atom<'a>,
    pub score: u32,
    pub shapes: Vec<AtomShape<'a>>
}

impl<'a> Atom<'a> {
    /// Construct an `Atom` without shape from its `AtomType`
    pub fn from_type(t: AtomType) -> Self {
        Self {
            t: t,
            shape: None
        }
    }

    /// Construct an `Atom` with a shape from its `AtomType`
    pub fn from_type_with_shape(t: AtomType, font: &'a Font, 
                                position: (f32, f32))        -> Self {
        let mut shape = AtomShape::from_atom_type(&t, font);
        shape.set_position(position);
        Self {
            t: t,
            shape: Some(shape)
        }
    }

    /// Return the atomic number of contained atom if regular, else `0`
    fn value(&self) -> u8 {
        match self.t {
            AtomType::Atom(z) => z,
            _ => 0
        }
    }

    /// Build a copy of a given `Atom`
    pub fn copy(other: &Self) -> Self {
        Self {
            t: other.t.clone(),
            shape: None,
        }
    }
}

impl<'a> GameState<'a> {
    /// Creates a new empty `GameState`.
    fn new() -> Self {
        GameState {
            atoms: Vec::new(),
            shift: 0,
            time: 0,
            incoming: Atom::from_type(AtomType::None),
            score: 0,
            shapes: Vec::new(),
        }
    }

    /// Creates the `GameState` for the beginning of the game. 
    pub fn start_game() -> Self {
        let mut new = GameState::new();
        new.atoms.extend_from_slice(&[
            Atom::from_type(AtomType::Atom(0)),
            Atom::from_type(AtomType::Atom(0)),
            Atom::from_type(AtomType::Atom(0)),
            Atom::from_type(AtomType::Atom(0))]);
        new.draw_incoming();
        new
    }

    /// Draws the incoming atom (overwrites the current one)
    pub fn draw_incoming(&mut self) {
        let r = rand::thread_rng().gen::<f64>();

        if r < PLUS_CHANCE {
            // plus atom drawn
            self.incoming = Atom::from_type(AtomType::Plus);
        } else if r < C2 {
            // minus atom drawn
            self.incoming = Atom::from_type(AtomType::Minus);
        } else if self.score >= MIN_DPLUS_SCORE && r < C3 {
            // dark plus atom drawn
            self.incoming = Atom::from_type(AtomType::DarkPlus);
        } else if self. score >= MIN_WHITE_SCORE && r < C4 {
            // white (neutrino) atom drawn
            self.incoming = Atom::from_type(AtomType::Neutrino);
        } else {
            // classic atom drawn

        }

        if BERN_02.sample(&mut rand::thread_rng()) {
            self.incoming = Atom::from_type(AtomType::Plus);
        }
        else if BERN_02.sample(&mut rand::thread_rng()) {
            self.incoming = Atom::from_type(AtomType::Minus);
        }
        else {
            let v = BINOM.sample(&mut rand::thread_rng());
            self.incoming = Atom::from_type(AtomType::Atom(v as u8));
        }
    }

    /// Shot the incoming atom at the n-th position and update the `GameState`
    ///
    /// Returns: the value of the highest atom that has reacted (may be 0 if 
    /// none)
    pub fn play(&mut self, k: u8) -> u8{
        let i = k as usize;
        self.atoms.insert(i, Atom::copy(&self.incoming));

        let n = self.atoms.len();
        let l = if i + 1 >= n { 0 } else { i + 1 };
        if self.atoms[l] < self.atoms[l] {
            self.reorder();
        }

        self.time += 1;
        let max = self.update_plus();
        self.draw_incoming();
        max
    }

    /// Makes the reactions with pluses atoms
    ///
    /// Returns: the value of the highest atom that has reacted (may be 0 if 
    /// none)
    pub fn update_plus(&mut self) -> u8 {
        let mut reaction = true;
        let mut max: u8 = 0;
        while reaction {
            reaction = false;
            let mut i = 0;
            while i < self.atoms.len() {
                if self.atoms[i].t == AtomType::Plus {
                    let m = self.react(i);
                    if  m > 0 {
                        reaction = true;
                        if m > max { max = m; }
                    }
                }
                i += 1;
            }
        }
        max
    }

    /// Attempts to react atom at index k
    pub fn react(&mut self, mut k: usize) -> u8 {
        let mut n = self.atoms.len();
        let mut k_prev = safe(k, -1, n);
        let mut k_next = safe(k, 1,  n);
        if k_prev == k_next { return 0; }
        k %= n;

        let mut score_multiplier = 1.5;

        let mut final_value: u8 = 0;
        while self.atoms[k_prev] == self.atoms[k_next] ||
              self.atoms[k].t == AtomType::DarkPlus {

            self.score += match self.atoms[k].t {
                AtomType::DarkPlus => {
                    let z_r = self.atoms[k_prev].value();
                    let z_l = self.atoms[k_next].value();
                    let z_max = max(z_r,z_l);

                    ((score_multiplier * (z_max as f64 + 1_f64)).floor()) 
                    as u32
                },
                AtomType::Plus => {
                    (score_multiplier * 
                    (self.atoms[k_prev].value() as f64 + 1_f64).floor()) as u32
                },
                AtomType::Atom(z_in) => {
                    let z_out = self.atoms[k_prev].value();
                    let reaction_score = score_multiplier * 
                        (z_out as f64 + 1_f64).floor();
                    let bonus = 2_f64 * score_multiplier * 
                        (z_out as f64 - z_in as f64 + 1_f64).floor();
                    
                    if z_out < z_in { reaction_score as u32 }
                    else            { (reaction_score + bonus) as u32}
                },
                _ => panic!("Reacting impossible stuff")
            };
            score_multiplier += 0.5;

            // println!("reacting ({}, {}, {})%{} two: {:?}", k_prev, k, k_next, n, self.atoms[k_prev].t);
            let (z_in, dark) = match self.atoms[k].t {
                AtomType::Atom(z) => (z, false),
                AtomType::DarkPlus => (0, true),
                _ => (0, false)
            };
            let z_out = match self.atoms[k_prev].t {
                AtomType::Atom(z) => z,
                _ => break
            };

            let z_f = if dark { 
                let z1 = self.atoms[k_prev].value();
                let z2 = self.atoms[k_next].value();
                if z1 > z2 { z1 + 3 }
                else       { z2 + 3 }
            } else {
                if z_out > z_in { z_out + 1 }
                else            { z_in + 1 }
            };

            self.atoms[k] = Atom::from_type(AtomType::Atom(z_f));
            self.atoms.remove(k_next);
            self.atoms.remove(safe(k, -1, n-1));
            final_value = z_f;
            n = self.atoms.len();
            if n == 0 {
                // println!("n == 0 ??? ow");
                return final_value;
            }

            // print!("reacted: {} ", k);
            k = if k == 0 { 0 } else { (k - 1) % n };
            k_prev = safe(k, -1, n);
            k_next = safe(k, 1,  n);
            // println!("now: (k_prev, k, k_next) = ({}, {}, {}) n: {}", k_prev, k, k_next, n);
            // println!("@k_prev: {:?}", self.atoms[k_prev].t);
            // println!("@k     : {:?}", self.atoms[k].t);
            // println!("@k_next: {:?}", self.atoms[k_next].t);
            // if k_next == k_prev { println!("breaking"); break; }
        }
        final_value
    }

    /// "Reorder" the `atoms` vector. In facts, finds the new `shift` value as
    /// the current one may no longer lead to the first ordering.
    fn reorder(&mut self) {
        let n = self.atoms.len();
        let mut best = self.shift;
        for i in 0..n {
            if self.is_better(i, best) {
                best = i;
            }
        }
        // print!("shift: {} → ", self.shift);
        self.shift = best;
        // println!("{}", self.shift);
    }

    /// Returns `true` if the ordering beginning at `i1` is less that the one
    /// beginning at `i2`.
    ///
    /// Returns `false` when equal.
    fn is_better(&self, i1: usize, i2: usize) -> bool {
        let n = self.atoms.len();
        for j in 0..n {
            if self.atoms[i1 + j] < self.atoms[i2 + j] {
                return true;
            } else if self.atoms[i1 + j] > self.atoms[i2 + j] {
                return false;
            }
        }
        false
    }

    /// Prints info about the state 
    pub fn info(&self) {
        println!("<state>");
        println!("shift: {}", self.shift);
        for i in 0..self.atoms.len() {
            println!("{}: {:?}", i, self.atoms[i].t);
        }
        println!("</state>")
        
    }
}

/// Helper function to add or subtract safely modulo n
fn safe(i: usize, di: i32, n: usize) -> usize {
    let i2 = i as i32;
    let n2 = n as i32;
    if i2 + di < 0 {
        ((i2 + n2 + di) % n2) as usize
    }
    else { 
        ((i2 + di) % n2) as usize
    }
}

impl std::cmp::PartialEq for AtomType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            AtomType::Plus => match other {
                AtomType::Plus => true,
                _ => false
            },
            AtomType::Minus => match other {
                AtomType::Minus => true,
                _ => false
            },
            AtomType::DarkPlus => match other {
                AtomType::DarkPlus => true,
                _ => false
            },
            AtomType::Atom(z1) => match other {
                AtomType::Atom(z2) => z1 == z2,
                _ => false
            },
            AtomType::None => match other {
                AtomType::None => true,
                _ => false
            },
            AtomType::Neutrino => match other {
                AtomType::Neutrino => true,
                _ => false
            }
        }
    }
}

impl std::cmp::PartialOrd for AtomType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for AtomType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            AtomType::None => match other {
                AtomType::None => Ordering::Equal,
                _ => Ordering::Less
            },
            AtomType::Plus => match other {
                AtomType::None => Ordering::Greater,
                AtomType::Plus => Ordering::Equal,
                _ => Ordering::Less
            }
            AtomType::Minus => match other {
                AtomType::None | AtomType::Plus => Ordering::Greater,
                AtomType::Minus => Ordering::Equal,
                _ => Ordering::Less
            }
            AtomType::DarkPlus => match other {
                AtomType::None | AtomType::Plus
                               | AtomType::Minus => Ordering::Greater,
                AtomType::DarkPlus => Ordering::Equal,
                _ => Ordering::Less
            }
            AtomType::Neutrino => match other {
                AtomType::Atom(_) => Ordering::Less,
                AtomType::Neutrino => Ordering::Equal,
                _ => Ordering::Greater
            }
            AtomType::Atom(z1) => match other {
                AtomType::Atom(z2) => z1.cmp(z2),
                _ => Ordering::Greater
            }
        }
    }
}

impl<'a> std::cmp::PartialEq for Atom<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t.eq(&other.t)
    }
}

impl<'a> std::cmp::PartialOrd for Atom<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> std::cmp::Ord for Atom<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.cmp(&other.t)
    }
}

impl<'a> std::cmp::Eq for Atom<'a> { }


impl<'a> std::cmp::PartialEq for GameState<'a> {
    /// Symmetrical states are considered equals.
    fn eq(&self, other: &Self) -> bool {
        let n = self.atoms.len();
        if n != other.atoms.len() { return false; }
        let mut clock_wise_ok = true;
        let mut cclock_wise_ok = true;
        for i in 0..n {
            if self.atoms[i] != other.atoms[i] {
                clock_wise_ok = false;
            }
            if self.atoms[i] != other.atoms[n - i] {
                cclock_wise_ok = false;
            }
            if !clock_wise_ok && !cclock_wise_ok {
                return false;
            }
        }
        true
    }
}
