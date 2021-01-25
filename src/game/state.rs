use rand_distr::{Binomial, Distribution};
use lazy_static::lazy_static;

lazy_static!{
    static ref BINOM: Binomial = Binomial::new(7, 0.5).unwrap();
}

/// Represents anything we can get when "drawing a new atom", that is, it takes
/// the `Atom`s, `Plus`, `Minus`, `DarkPlus` into account.
///
/// Antimatter will not be taken into account.
#[derive(Debug, Clone)]
pub enum Atom {
    Plus, 
    DarkPlus,
    Minus,
    Atom(u8),
    None,
}

/// Represents the state of the game at some point.
///
/// Contains:
///
/// - `atoms`: a vector of `Atom`s
///
/// - `shift`: the shift, that is, as the vector of atoms is the least "word" 
/// of every possible rotation, as we consider two rotation of the same atoms 
/// to be the same, and symmetry doesn't matter neither.
///
/// - `time`: the number of turns played since the start
///
/// - `incoming`: an option containing the incoming atom.
///
/// - `score`: the score so far
#[derive(Debug, Clone)]
pub struct GameState {
    pub atoms: Vec<Atom>,
    pub shift: usize,
    pub time: u32,
    pub incoming: Atom,
    pub score: u32,
}

impl GameState {
    /// Creates a new empty `GameState`.
    fn new() -> Self {
        GameState {
            atoms: Vec::new(),
            shift: 0,
            time: 0,
            incoming: Atom::None,
            score: 0
        }
    }

    /// Creates the `GameState` for the beginning of the game. 
    pub fn start_game() -> Self {
        let mut new = GameState::new();
        new.atoms.extend_from_slice(&[Atom::Atom(0), Atom::Atom(0),
                                      Atom::Atom(1), Atom::Atom(0)]);
        new.draw_incoming();
        new
    }

    /// Draws the incoming atom (if already, do nothing)
    pub fn draw_incoming(&mut self) {
        let v = BINOM.sample(&mut rand::thread_rng());
        self.incoming = Atom::Atom(v as u8);
    }
}

impl std::cmp::PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Atom::Plus => match other {
                Atom::Plus => true,
                _ => false
            },
            Atom::Minus => match other {
                Atom::Minus => true,
                _ => false
            },
            Atom::DarkPlus => match other {
                Atom::DarkPlus => true,
                _ => false
            },
            Atom::Atom(z1) => match other {
                Atom::Atom(z2) => z1 == z2,
                _ => false
            },
            Atom::None => match other {
                Atom::None => true,
                _ => false
            }
        }
    }
}

impl std::cmp::PartialEq for GameState {
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