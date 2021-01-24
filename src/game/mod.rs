mod state;

extern crate lazy_static;

use state::GameState;
use std::sync::Mutex;

use gtk::{
    Box,
    ContainerExt,
    DrawingArea,
    GtkWindowExt,
    WidgetExt
};

use lazy_static::lazy_static;


struct StatePointer<'a> {
    state: Option<&'a GameState>,
    x: f64
}

lazy_static!{
    static ref STATE_POINTER: Mutex<StatePointer<'static>> = Mutex::new(StatePointer::new());
}

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("IAtomas");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(400, 500);

    let container = Box::new(gtk::Orientation::Vertical, 5);

    let button = gtk::Button::with_label("Click A!");
    container.add(&button);

    let da = DrawingArea::new();
    da.set_size_request(400, 400);
    da.connect_draw(move |_w, c| {
        let x = function();
        c.set_source_rgb(0_f64, 0.8_f64, 1.0_f64);
        c.rectangle(x, 0_f64, 200_f64, 200_f64);
        c.fill();
        println!("{}", STATE_POINTER.lock().unwrap().get_x());
        gtk::Inhibit(false)
    });

    da.connect_button_release_event(|_a, _b| {
        gtk::Inhibit(false)
    });
    container.add(&da);

    window.add(&container);

    window.show_all();
}

fn function() -> f64 {
    STATE_POINTER.lock().unwrap().add_x(1_f64);
    STATE_POINTER.lock().unwrap().get_x()
}

impl StatePointer<'_> {
    fn new() -> Self {
        StatePointer {
            state: None,
            x: 100_f64
        }
    }

    fn get_x(&self) -> f64 {
        self.x
    }

    fn add_x(&mut self, y: f64) {
        self.x = self.x + y;
    }
}

pub const AtomsSymbols: [&str] = [
    "H",  "He", "Li", "Be", "B",  "C",  "N",  "O",  "F",  "Ne", "Na", "Mg", 
    "Al", "Si", "P",  "S",  "Cl", "Ar", "K",  "Ca", "Sc", "Ti", "Va", "Cr", 
    "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As", "Se", "Br", "Kr",
    "Rb", "Sr", "Y",  "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", 
    "In", "Sn", "Sb", "Te", "I",  "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd",
    "Pm", "Sm", "Eu", "Gb", "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", 
    "Ta", "W",  "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi", "Po",
    "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa", "U",  "Np", "Pu", "Am", "Cm",
    "Bk", "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs",
    "Mt", "Ds", "Rg", "Cn", "Nh", "Fl", "Mc", "Lv", "Ts", "Og" ];

pub const AtomsNames: [&str] = [
    "Hydrogen",    "Helium",     "Lithium",    "Beryllium",     "Boron",
    "Carbon",      "Nitrogen",   "Oxygen",     "Fluorine",      "Neon",
    "Sodium",      "Magnesium",  "Aluminum",   "Silicon",       "Phosphorus",
    "Sulfur",      "Chlorine",   "Argon",      "Potassium",     "Calcium",
    "Scandium",    "Titanium",   "Vanadium",   "Chromium",      "Manganese",
    "Iron",        "Cobalt",     "Nickel",     "Copper",        "Zinc",
    "Gallium",     "Germanium",  "Arsenic",    "Selenium",      "Bromine",
    "Krypton",     "Rubidium",   "Strontium",  "Yttrium",       "Zirconium",
    "Niobium",     "Molybdenum", "Technetium", "Ruthenium",     "Rhodium",
    "Palladium",   "Silver",     "Cadmium",    "Indium",        "Tin",    
    "Antimony",    "Tellurium",  "Iodine",     "Xenon",         "Cesium",        
    "Barium",      "Lanthanum",  "Cerium",     "Praseodymium",  "Neodymium",
    "Promethium",  "Samarium",   "Europium",   "Gadolinium",    "Terbium",
    "Dysprosium",  "Holmium",    "Erbium",     "Thulium",       "Ytterbium",
    "Lutetium",    "Hafnium",    "Tantalum",   "Tungsten",      "Rhenium",
    "Osmium",      "Iridium",    "Platinum",   "Gold",          "Mercury",
    "Thallium",    "Lead",       "Bismuth",    "Polonium",      "Astatine",
    "Radon",       "Francium",   "Radium",     "Actinium",      "Thorium",
    "Protactinium","Uranium",    "Neptunium",  "Plutonium",     "Americium",
    "Curium",      "Berkelium",  "Californium","Einsteinium",   "Fermium",
    "Mendelevium", "Nobelium",   "Lawrencium", "Rutherfordium", "Dubnium",
    "Seaborgium",  "Bohrium",    "Hassium",    "Meitnerium",    "Darmstadtium",
    "Roentgenium", "Copernicium","Nihonium",   "Flerovium",     "Moscovium",
    "Livermorium", "Tennessine", "Oganesson" ];