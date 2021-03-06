/// Array of all the atoms' symbols.
///
/// Extended with Atomas' specific atoms :)
pub const ATOMS_SYMBOLS: [&str; 131] = [
    "H",  "He", "Li", "Be", "B",  "C",  "N",  "O",  "F",  "Ne", "Na", "Mg", 
    "Al", "Si", "P",  "S",  "Cl", "Ar", "K",  "Ca", "Sc", "Ti", "Va", "Cr", 
    "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As", "Se", "Br", "Kr",
    "Rb", "Sr", "Y",  "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", 
    "In", "Sn", "Sb", "Te", "I",  "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd",
    "Pm", "Sm", "Eu", "Gb", "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", 
    "Ta", "W",  "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi", "Po",
    "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa", "U",  "Np", "Pu", "Am", "Cm",
    "Bk", "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs",
    "Mt", "Ds", "Rg", "Cn", "Nh", "Fl", "Mc", "Lv", "Ts", "Og", "Bn", "Gb",
    "Bb", "Pi", "Sir", "Ea", "Ubp", "H1", "He1", "Li1", "Be1", "B1", "C1" ];

/// Array of all the atoms' full names.
///
/// Extended with Atomas' specific atoms :)
pub const ATOMS_NAMES: [&str; 131] = [
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
    "Livermorium", "Tennessine", "Oganesson",  "Bananium",      "GravityBlockium",
    "BreakingBadium", "314159265359",          "Sirnicanium",   "Earthium",
    "Unbipentium",  "Hydrogen 1", "Helium 1",  "Lithium 1",     "Beryllium 1",
    "Boron 1",      "Carbon 1" ];

/// Array of all the atoms' color, in formated hexcode (see 
/// `crate::game::color_from_hex` function).
pub const ATOMS_COLORS: [&str; 131] = [
    "#63b9d5",  "#d1c991",  "#4c6168",  "#c8c8c8",  "#7d5353",  "#3b3b3b",
    "#2cc6b2",  "#6fec98",  "#ecc46f",  "#be0086",  "#e69d7a",  "#9e80ea", 
    "#797979",  "#4a4070",  "#d7463f",  "#375e7c",  "#6d1d7b",  "#9a3da5", 
    "#4d8946",  "#f0f0f0",  "#5fbb77",  "#5a5a5a",  "#5f9ebb",  "#a488b5", 
    "#dc4a4a",  "#ab967d",  "#4371e6",  "#bac395",  "#b95739",  "#b4b4b4", 
    "#39b975",  "#979273",  "#738498",  "#424242",  "#d4753c",  "#3ca0d4", 
    "#d22c1f",  "#ff9d29",  "#b129ff",  "#d6e43a",  "#75dceb",  "#8ba38c", 
    "#eea1e2",  "#563e32",  "#88d17a",  "#9eabbe",  "#dcdcdc",  "#5560c8", 
    "#408d3c",  "#b5a47c",  "#c6598c",  "#827498",  "#ff00fc",  "#7888ff", 
    "#ffd478",  "#e99c9c",  "#8bdbbe",  "#ff9329",  "#56e019",  "#65898d", 
    "#2ee99b",  "#bd6475",  "#6c64bd",  "#6e1289",  "#359c50",  "#447ee7", 
    "#e77d46",  "#bf4987",  "#21426b",  "#878750",  "#d12c2c",  "#91d12c", 
    "#7f87af",  "#2b6aa5",  "#512f2f",  "#307060",  "#c9876a",  "#505008", 
    "#edc474",  "#80a5ac",  "#ac8089",  "#3c7c66",  "#ff0506",  "#ffff00", 
    "#00ff00",  "#dae83a",  "#ff6c00",  "#00ffff",  "#424918",  "#aa3d82", 
    "#3daa82",  "#9cff00",  "#00aeff",  "#ff9000",  "#813349",  "#ff79d0", 
    "#ae877e",  "#8f3cb4",  "#86c4dc",  "#bfdc86",  "#dc8686",  "#ffd965", 
    "#5c24a0",  "#6b6675",  "#b05032",  "#254987",  "#9bafa0",  "#ff562d", 
    "#cdcd2c",  "#3a7e48",  "#0000ff",  "#aa4594",  "#8f8f8f",  "#2eede6", 
    "#beaf64",  "#f22e6a",  "#70ea78",  "#ff00b9",  "#ede674",  "#3de6c3", 
    "#309141",  "#4dc8e6",  "#ff0000",  "#1177f5",  "#000000",  "#ff4561",
    "#fff123",  "#ff3c21",   "#fd5122", "#fbf551",  "#424141" ];