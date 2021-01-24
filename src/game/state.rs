
type Atom = u8;
pub struct GameState {
    atoms: Vec<Atom>,
    shift: usize,
    time: u32,
}

impl GameState {
    fn new() -> Self {
        GameState {
            atoms: Vec::new(),
            shift: 0,
            time: 0
        }
    }
}

impl std::cmp::PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        let n = self.atoms.len();
        if n != other.atoms.len() { return false; }
        let mut clock_wise_ok = true;
        let mut cclock_wise_ok = true;
        for i in 0..n {
            let k = if i + shift >= n { i + shift - n } else { i + shift };
            if self.atoms[i] != other.atoms[k] {
                clock_wise_ok = false;
            }
            let k = if shift - i < 0 { shift + n - i } else { shift - i };
            if self.atoms[i] != other.atoms[k] {
                cclock_wise_ok = false;
            }
            if !clock_wise_ok && !clock_wise_ok {
                return false;
            }
        }
    }
}