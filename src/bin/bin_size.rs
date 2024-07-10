use std::time::Instant;

struct Hydrogen {}
struct Helium {}
struct Lithium {}
struct Beryllium {}
struct Boron {}
struct Carbon {}
struct Nitrogen {}
struct Oxygen {}
struct Fluorine {}
struct Neon {}
struct Sodium {}
struct Magnesium {}
struct Aluminum {}
struct Silicon {}
struct Phosphorus {}
struct Sulfur {}
struct Chlorine {}
struct Argon {}
struct Potassium {}
struct Calcium {}
struct Scandium {}
struct Titanium {}
struct Vanadium {}
struct Chromium {}
struct Manganese {}
struct Iron {}
struct Cobalt {}
struct Nickel {}
struct Copper {}
struct Zinc {}
struct Gallium {}
struct Germanium {}
struct Arsenic {}
struct Selenium {}
struct Bromium {}
struct Krypton {}
struct Rubidium {}
struct Strontium {}
struct Yttrium {}
struct Zirconium {}
struct Niobium {}
struct Molybdenum {}
struct Technetium {}
struct Ruthenium {}
struct Rhodium {}
struct Palladium {}
struct Silver {}
struct Cadmium {}
struct Indium {}
struct Tin {}
struct Antimony {}
struct Tellurium {}
struct Iodine {}
struct Xenon {}
struct Cesium {}
struct Barium {}
struct Lanthanum {}
struct Cerium {}
struct Prazeodymium {}
struct Neodymium {}
struct Promethium {}
struct Sammarium {}
struct Europium {}
struct Gadolium {}
struct Terbium {}
struct Dysprosium {}
struct Holmium {}
struct Erbium {}
struct Thulium {}
struct Ytterbium {}
struct Lutetium {}
struct Hafnium {}
struct Tantalum {}
struct Tungsten {}
struct Rhenium {}
struct Osmium {}
struct Ir {}
struct Platinum {}
struct Gold {}
struct Mercury {}
struct Thallium {}
struct Lead {}
struct Bismuth {}
struct Polonium {}
struct Astatine {}
struct Radon {}
struct Fr {}
struct Radium {}
struct Actinium {}
struct Thorium {}
struct Protactinium {}
struct Uranium {}
struct Neptunium {}
struct Plutonium {}
struct Americium {}
struct Curium {}
struct Berkelium {}
struct Californium {}
struct Einstein {}
struct Fermium {}
struct Mendelevium {}
struct Nobelium {}
struct Larensium {}
struct Rutherford {}
struct Dubnium {}
struct Seaborgium {}
struct Bohr {}
struct Hassium {}
struct Meitnerium {}
struct Darmstad {}
struct Roentgenium {}
struct Copernicus {}
struct Nihon {}
struct Flerovium {}
struct Moscovium {}
struct Livermorium {}
struct Tenes {}
struct Oganesson {}


trait ChemicalElement {
    fn atomic_number(&self) -> f32;
    fn short_name(&self) -> &str;
}


impl ChemicalElement for Hydrogen {
    fn atomic_number(&self) -> f32 {
        1.008f32
    }
    fn short_name(&self) -> &str {
        "H"
    }
}

impl ChemicalElement for Helium {
    fn atomic_number(&self) -> f32 {
        4.0026f32
    }
    fn short_name(&self) -> &str {
        "He"
    }
}

impl ChemicalElement for Lithium {
    fn atomic_number(&self) -> f32 {
        6.94f32
    }
    fn short_name(&self) -> &str {
        "Li"
    }
}

impl ChemicalElement for Beryllium {
    fn atomic_number(&self) -> f32 {
        9.0122f32
    }
    fn short_name(&self) -> &str {
        "Be"
    }
}

impl ChemicalElement for Boron {
    fn atomic_number(&self) -> f32 {
        10.81f32
    }
    fn short_name(&self) -> &str {
        "B"
    }
}

impl ChemicalElement for Carbon {
    fn atomic_number(&self) -> f32 {
        12.011f32
    }
    fn short_name(&self) -> &str {
        "C"
    }
}

impl ChemicalElement for Nitrogen {
    fn atomic_number(&self) -> f32 {
        14.007f32
    }
    fn short_name(&self) -> &str {
        "N"
    }
}

impl ChemicalElement for Oxygen {
    fn atomic_number(&self) -> f32 {
        15.999f32
    }
    fn short_name(&self) -> &str {
        "O"
    }
}

impl ChemicalElement for Fluorine {
    fn atomic_number(&self) -> f32 {
        18.998f32
    }
    fn short_name(&self) -> &str {
        "F"
    }
}

impl ChemicalElement for Neon {
    fn atomic_number(&self) -> f32 {
        20.180f32
    }
    fn short_name(&self) -> &str {
        "Ne"
    }
}

impl ChemicalElement for Sodium {
    fn atomic_number(&self) -> f32 {
        22.990f32
    }
    fn short_name(&self) -> &str {
        "Na"
    }
}

impl ChemicalElement for Magnesium {
    fn atomic_number(&self) -> f32 {
        24.305f32
    }
    fn short_name(&self) -> &str {
        "Mg"
    }
}

impl ChemicalElement for Aluminum {
    fn atomic_number(&self) -> f32 {
        26.982f32
    }
    fn short_name(&self) -> &str {
        "Al"
    }
}

impl ChemicalElement for Silicon {
    fn atomic_number(&self) -> f32 {
        28.085f32
    }
    fn short_name(&self) -> &str {
        "Si"
    }
}

impl ChemicalElement for Phosphorus {
    fn atomic_number(&self) -> f32 {
        30.974f32
    }
    fn short_name(&self) -> &str {
        "P"
    }
}

impl ChemicalElement for Sulfur {
    fn atomic_number(&self) -> f32 {
        32.06f32
    }
    fn short_name(&self) -> &str {
        "S"
    }
}

impl ChemicalElement for Chlorine {
    fn atomic_number(&self) -> f32 {
        35.45f32
    }
    fn short_name(&self) -> &str {
        "Cl"
    }
}

impl ChemicalElement for Argon {
    fn atomic_number(&self) -> f32 {
        39.948f32
    }
    fn short_name(&self) -> &str {
        "Ar"
    }
}

impl ChemicalElement for Potassium {
    fn atomic_number(&self) -> f32 {
        39.098f32
    }
    fn short_name(&self) -> &str {
        "K"
    }
}

impl ChemicalElement for Calcium {
    fn atomic_number(&self) -> f32 {
        40.078f32
    }
    fn short_name(&self) -> &str {
        "Ca"
    }
}

impl ChemicalElement for Scandium {
    fn atomic_number(&self) -> f32 {
        44.956f32
    }
    fn short_name(&self) -> &str {
        "Sc"
    }
}

impl ChemicalElement for Titanium {
    fn atomic_number(&self) -> f32 {
        47.867f32
    }
    fn short_name(&self) -> &str {
        "Ti"
    }
}

impl ChemicalElement for Vanadium {
    fn atomic_number(&self) -> f32 {
        50.942f32
    }
    fn short_name(&self) -> &str {
        "V"
    }
}

impl ChemicalElement for Chromium {
    fn atomic_number(&self) -> f32 {
        51.996f32
    }
    fn short_name(&self) -> &str {
        "Cr"
    }
}

impl ChemicalElement for Manganese {
    fn atomic_number(&self) -> f32 {
        54.938f32
    }
    fn short_name(&self) -> &str {
        "Mn"
    }
}

impl ChemicalElement for Iron {
    fn atomic_number(&self) -> f32 {
        55.845f32
    }
    fn short_name(&self) -> &str {
        "Fe"
    }
}

impl ChemicalElement for Cobalt {
    fn atomic_number(&self) -> f32 {
        58.933f32
    }
    fn short_name(&self) -> &str {
        "Co"
    }
}

impl ChemicalElement for Nickel {
    fn atomic_number(&self) -> f32 {
        58.693f32
    }
    fn short_name(&self) -> &str {
        "Ni"
    }
}

impl ChemicalElement for Copper {
    fn atomic_number(&self) -> f32 {
        63.546f32
    }
    fn short_name(&self) -> &str {
        "Cu"
    }
}

impl ChemicalElement for Zinc {
    fn atomic_number(&self) -> f32 {
        65.38f32
    }
    fn short_name(&self) -> &str {
        "Zn"
    }
}

impl ChemicalElement for Gallium {
    fn atomic_number(&self) -> f32 {
        69.723f32
    }
    fn short_name(&self) -> &str {
        "Ga"
    }
}

impl ChemicalElement for Germanium {
    fn atomic_number(&self) -> f32 {
        72.63f32
    }
    fn short_name(&self) -> &str {
        "Ge"
    }
}

impl ChemicalElement for Arsenic {
    fn atomic_number(&self) -> f32 {
        74.922f32
    }
    fn short_name(&self) -> &str {
        "As"
    }
}

impl ChemicalElement for Selenium {
    fn atomic_number(&self) -> f32 {
        78.971f32
    }
    fn short_name(&self) -> &str {
        "Se"
    }
}

impl ChemicalElement for Bromium {
    fn atomic_number(&self) -> f32 {
        79.904f32
    }
    fn short_name(&self) -> &str {
        "Br"
    }
}

impl ChemicalElement for Krypton {
    fn atomic_number(&self) -> f32 {
        83.798f32
    }
    fn short_name(&self) -> &str {
        "Kr"
    }
}

impl ChemicalElement for Rubidium {
    fn atomic_number(&self) -> f32 {
        85.468f32
    }
    fn short_name(&self) -> &str {
        "Rb"
    }
}

impl ChemicalElement for Strontium {
    fn atomic_number(&self) -> f32 {
        87.62f32
    }
    fn short_name(&self) -> &str {
        "Sr"
    }
}

impl ChemicalElement for Yttrium {
    fn atomic_number(&self) -> f32 {
        88.906f32
    }
    fn short_name(&self) -> &str {
        "Y"
    }
}

impl ChemicalElement for Zirconium {
    fn atomic_number(&self) -> f32 {
        91.224f32
    }
    fn short_name(&self) -> &str {
        "Zr"
    }
}

impl ChemicalElement for Niobium {
    fn atomic_number(&self) -> f32 {
        92.906f32
    }
    fn short_name(&self) -> &str {
        "Nb"
    }
}

impl ChemicalElement for Molybdenum {
    fn atomic_number(&self) -> f32 {
        95.95f32
    }
    fn short_name(&self) -> &str {
        "Mo"
    }
}

impl ChemicalElement for Technetium {
    fn atomic_number(&self) -> f32 {
        98f32
    }
    fn short_name(&self) -> &str {
        "Tc"
    }
}

impl ChemicalElement for Ruthenium {
    fn atomic_number(&self) -> f32 {
        101.07f32
    }
    fn short_name(&self) -> &str {
        "Ru"
    }
}

impl ChemicalElement for Rhodium {
    fn atomic_number(&self) -> f32 {
        102.91f32
    }
    fn short_name(&self) -> &str {
        "Rh"
    }
}

impl ChemicalElement for Palladium {
    fn atomic_number(&self) -> f32 {
        106.42f32
    }
    fn short_name(&self) -> &str {
        "Pd"
    }
}

impl ChemicalElement for Silver {
    fn atomic_number(&self) -> f32 {
        107.87f32
    }
    fn short_name(&self) -> &str {
        "Ag"
    }
}

impl ChemicalElement for Cadmium {
    fn atomic_number(&self) -> f32 {
        112.41f32
    }
    fn short_name(&self) -> &str {
        "Cd"
    }
}

impl ChemicalElement for Indium {
    fn atomic_number(&self) -> f32 {
        114.82f32
    }
    fn short_name(&self) -> &str {
        "In"
    }
}

impl ChemicalElement for Tin {
    fn atomic_number(&self) -> f32 {
        118.71f32
    }
    fn short_name(&self) -> &str {
        "Sn"
    }
}

impl ChemicalElement for Antimony {
    fn atomic_number(&self) -> f32 {
        121.76f32
    }
    fn short_name(&self) -> &str {
        "Sb"
    }
}

impl ChemicalElement for Tellurium {
    fn atomic_number(&self) -> f32 {
        127.60f32
    }
    fn short_name(&self) -> &str {
        "Te"
    }
}

impl ChemicalElement for Iodine {
    fn atomic_number(&self) -> f32 {
        126.90f32
    }
    fn short_name(&self) -> &str {
        "I"
    }
}

impl ChemicalElement for Xenon {
    fn atomic_number(&self) -> f32 {
        131.29f32
    }
    fn short_name(&self) -> &str {
        "Xe"
    }
}

impl ChemicalElement for Cesium {
    fn atomic_number(&self) -> f32 {
        132.91f32
    }
    fn short_name(&self) -> &str {
        "Cs"
    }
}

impl ChemicalElement for Barium {
    fn atomic_number(&self) -> f32 {
        137.33f32
    }
    fn short_name(&self) -> &str {
        "Ba"
    }
}

impl ChemicalElement for Lanthanum {
    fn atomic_number(&self) -> f32 {
        138.91f32
    }
    fn short_name(&self) -> &str {
        "La"
    }
}

impl ChemicalElement for Cerium {
    fn atomic_number(&self) -> f32 {
        140.12f32
    }
    fn short_name(&self) -> &str {
        "Ce"
    }
}

impl ChemicalElement for Prazeodymium {
    fn atomic_number(&self) -> f32 {
        140.91f32
    }
    fn short_name(&self) -> &str {
        "Pr"
    }
}

impl ChemicalElement for Neodymium {
    fn atomic_number(&self) -> f32 {
        144.24f32
    }
    fn short_name(&self) -> &str {
        "Nd"
    }
}

impl ChemicalElement for Promethium {
    fn atomic_number(&self) -> f32 {
        145f32
    }
    fn short_name(&self) -> &str {
        "Pm"
    }
}

impl ChemicalElement for Sammarium {
    fn atomic_number(&self) -> f32 {
        150.36f32
    }
    fn short_name(&self) -> &str {
        "Sm"
    }
}

impl ChemicalElement for Europium {
    fn atomic_number(&self) -> f32 {
        151.96f32
    }
    fn short_name(&self) -> &str {
        "Eu"
    }
}

impl ChemicalElement for Gadolium {
    fn atomic_number(&self) -> f32 {
        157.25f32
    }
    fn short_name(&self) -> &str {
        "Gd"
    }
}

impl ChemicalElement for Terbium {
    fn atomic_number(&self) -> f32 {
        158.93f32
    }
    fn short_name(&self) -> &str {
        "Tb"
    }
}

impl ChemicalElement for Dysprosium {
    fn atomic_number(&self) -> f32 {
        162.50f32
    }
    fn short_name(&self) -> &str {
        "Dy"
    }
}

impl ChemicalElement for Holmium {
    fn atomic_number(&self) -> f32 {
        164.93f32
    }
    fn short_name(&self) -> &str {
        "Ho"
    }
}

impl ChemicalElement for Erbium {
    fn atomic_number(&self) -> f32 {
        167.26f32
    }
    fn short_name(&self) -> &str {
        "Er"
    }
}

impl ChemicalElement for Thulium {
    fn atomic_number(&self) -> f32 {
        168.93f32
    }
    fn short_name(&self) -> &str {
        "Tm"
    }
}

impl ChemicalElement for Ytterbium {
    fn atomic_number(&self) -> f32 {
        173.04f32
    }
    fn short_name(&self) -> &str {
        "Yb"
    }
}

impl ChemicalElement for Lutetium {
    fn atomic_number(&self) -> f32 {
        174.97f32
    }
    fn short_name(&self) -> &str {
        "Lu"
    }
}

impl ChemicalElement for Hafnium {
    fn atomic_number(&self) -> f32 {
        178.49f32
    }
    fn short_name(&self) -> &str {
        "Hf"
    }
}

impl ChemicalElement for Tantalum {
    fn atomic_number(&self) -> f32 {
        180.95f32
    }
    fn short_name(&self) -> &str {
        "Ta"
    }
}

impl ChemicalElement for Tungsten {
    fn atomic_number(&self) -> f32 {
        183.84f32
    }
    fn short_name(&self) -> &str {
        "W"
    }
}

impl ChemicalElement for Rhenium {
    fn atomic_number(&self) -> f32 {
        186.21f32
    }
    fn short_name(&self) -> &str {
        "Re"
    }
}

impl ChemicalElement for Osmium {
    fn atomic_number(&self) -> f32 {
        190.23f32
    }
    fn short_name(&self) -> &str {
        "Os"
    }
}

impl ChemicalElement for Ir {
    fn atomic_number(&self) -> f32 {
        192.22f32
    }
    fn short_name(&self) -> &str {
        "Ir"
    }
}

impl ChemicalElement for Platinum {
    fn atomic_number(&self) -> f32 {
        195.08f32
    }
    fn short_name(&self) -> &str {
        "Pt"
    }
}

impl ChemicalElement for Gold {
    fn atomic_number(&self) -> f32 {
        196.97f32
    }
    fn short_name(&self) -> &str {
        "Au"
    }
}

impl ChemicalElement for Mercury {
    fn atomic_number(&self) -> f32 {
        200.59f32
    }
    fn short_name(&self) -> &str {
        "Hg"
    }
}

impl ChemicalElement for Thallium {
    fn atomic_number(&self) -> f32 {
        204.38f32
    }
    fn short_name(&self) -> &str {
        "Tl"
    }
}

impl ChemicalElement for Lead {
    fn atomic_number(&self) -> f32 {
        207.2f32
    }
    fn short_name(&self) -> &str {
        "Pb"
    }
}

impl ChemicalElement for Bismuth {
    fn atomic_number(&self) -> f32 {
        208.98f32
    }
    fn short_name(&self) -> &str {
        "Bi"
    }
}

impl ChemicalElement for Polonium {
    fn atomic_number(&self) -> f32 {
        209f32
    }
    fn short_name(&self) -> &str {
        "Po"
    }
}

impl ChemicalElement for Astatine {
    fn atomic_number(&self) -> f32 {
        210f32
    }
    fn short_name(&self) -> &str {
        "At"
    }
}

impl ChemicalElement for Radon {
    fn atomic_number(&self) -> f32 {
        222f32
    }
    fn short_name(&self) -> &str {
        "Rn"
    }
}

impl ChemicalElement for Fr {
    fn atomic_number(&self) -> f32 {
        223f32
    }
    fn short_name(&self) -> &str {
        "Fr"
    }
}

impl ChemicalElement for Radium {
    fn atomic_number(&self) -> f32 {
        226f32
    }
    fn short_name(&self) -> &str {
        "Ra"
    }
}

impl ChemicalElement for Actinium {
    fn atomic_number(&self) -> f32 {
        227f32
    }
    fn short_name(&self) -> &str {
        "Ac"
    }
}

impl ChemicalElement for Thorium {
    fn atomic_number(&self) -> f32 {
        232.04f32
    }
    fn short_name(&self) -> &str {
        "Th"
    }
}

impl ChemicalElement for Protactinium {
    fn atomic_number(&self) -> f32 {
        231.04f32
    }
    fn short_name(&self) -> &str {
        "Pa"
    }
}

impl ChemicalElement for Uranium {
    fn atomic_number(&self) -> f32 {
        238.03f32
    }
    fn short_name(&self) -> &str {
        "U"
    }
}

impl ChemicalElement for Neptunium {
    fn atomic_number(&self) -> f32 {
        237f32
    }
    fn short_name(&self) -> &str {
        "Np"
    }
}

impl ChemicalElement for Plutonium {
    fn atomic_number(&self) -> f32 {
        244f32
    }
    fn short_name(&self) -> &str {
        "Pu"
    }
}

impl ChemicalElement for Americium {
    fn atomic_number(&self) -> f32 {
        243f32
    }
    fn short_name(&self) -> &str {
        "Am"
    }
}

impl ChemicalElement for Curium {
    fn atomic_number(&self) -> f32 {
        247f32
    }
    fn short_name(&self) -> &str {
        "Cm"
    }
}

impl ChemicalElement for Berkelium {
    fn atomic_number(&self) -> f32 {
        247f32
    }
    fn short_name(&self) -> &str {
        "Bk"
    }
}

impl ChemicalElement for Californium {
    fn atomic_number(&self) -> f32 {
        251f32
    }
    fn short_name(&self) -> &str {
        "Cf"
    }
}

impl ChemicalElement for Einstein {
    fn atomic_number(&self) -> f32 {
        252f32
    }
    fn short_name(&self) -> &str {
        "Es"
    }
}

impl ChemicalElement for Fermium {
    fn atomic_number(&self) -> f32 {
        257f32
    }
    fn short_name(&self) -> &str {
        "Fm"
    }
}

impl ChemicalElement for Mendelevium {
    fn atomic_number(&self) -> f32 {
        258f32
    }
    fn short_name(&self) -> &str {
        "Md"
    }
}

impl ChemicalElement for Nobelium {
    fn atomic_number(&self) -> f32 {
        259f32
    }
    fn short_name(&self) -> &str {
        "No"
    }
}

impl ChemicalElement for Larensium {
    fn atomic_number(&self) -> f32 {
        262f32
    }
    fn short_name(&self) -> &str {
        "Lr"
    }
}

impl ChemicalElement for Rutherford {
    fn atomic_number(&self) -> f32 {
        267f32
    }
    fn short_name(&self) -> &str {
        "Rf"
    }
}

impl ChemicalElement for Dubnium {
    fn atomic_number(&self) -> f32 {
        270f32
    }
    fn short_name(&self) -> &str {
        "Db"
    }
}

impl ChemicalElement for Seaborgium {
    fn atomic_number(&self) -> f32 {
        271f32
    }
    fn short_name(&self) -> &str {
        "Sg"
    }
}

impl ChemicalElement for Bohr {
    fn atomic_number(&self) -> f32 {
        270f32
    }
    fn short_name(&self) -> &str {
        "Bh"
    }
}

impl ChemicalElement for Hassium {
    fn atomic_number(&self) -> f32 {
        277f32
    }
    fn short_name(&self) -> &str {
        "Hs"
    }
}

impl ChemicalElement for Meitnerium {
    fn atomic_number(&self) -> f32 {
        278f32
    }
    fn short_name(&self) -> &str {
        "Mt"
    }
}

impl ChemicalElement for Darmstad {
    fn atomic_number(&self) -> f32 {
        281f32
    }
    fn short_name(&self) -> &str {
        "Ds"
    }
}

impl ChemicalElement for Roentgenium {
    fn atomic_number(&self) -> f32 {
        282f32
    }
    fn short_name(&self) -> &str {
        "Rg"
    }
}

impl ChemicalElement for Copernicus {
    fn atomic_number(&self) -> f32 {
        285f32
    }
    fn short_name(&self) -> &str {
        "Cn"
    }
}

impl ChemicalElement for Nihon {
    fn atomic_number(&self) -> f32 {
        286f32
    }
    fn short_name(&self) -> &str {
        "Nh"
    }
}

impl ChemicalElement for Flerovium {
    fn atomic_number(&self) -> f32 {
        289f32
    }
    fn short_name(&self) -> &str {
        "Fl"
    }
}

impl ChemicalElement for Moscovium {
    fn atomic_number(&self) -> f32 {
        290f32
    }
    fn short_name(&self) -> &str {
        "Mc"
    }
}

impl ChemicalElement for Livermorium {
    fn atomic_number(&self) -> f32 {
        293f32
    }
    fn short_name(&self) -> &str {
        "Lv"
    }
}

impl ChemicalElement for Tenes {
    fn atomic_number(&self) -> f32 {
        294f32
    }
    fn short_name(&self) -> &str {
        "Ts"
    }
}

impl ChemicalElement for Oganesson {
    fn atomic_number(&self) -> f32 {
        294f32
    }
    fn short_name(&self) -> &str {
        "Og"
    }
}


fn instantiate_elements() -> Vec<Box<dyn ChemicalElement>> {
    vec![
        Box::new(Hydrogen {}),
        Box::new(Helium {}),
        Box::new(Lithium {}),
        Box::new(Beryllium {}),
        Box::new(Boron {}),
        Box::new(Carbon {}),
        Box::new(Nitrogen {}),
        Box::new(Oxygen {}),
        Box::new(Fluorine {}),
        Box::new(Neon {}),
        Box::new(Sodium {}),
        Box::new(Magnesium {}),
        Box::new(Aluminum {}),
        Box::new(Silicon {}),
        Box::new(Phosphorus {}),
        Box::new(Sulfur {}),
        Box::new(Chlorine {}),
        Box::new(Argon {}),
        Box::new(Potassium {}),
        Box::new(Calcium {}),
        Box::new(Scandium {}),
        Box::new(Titanium {}),
        Box::new(Vanadium {}),
        Box::new(Chromium {}),
        Box::new(Manganese {}),
        Box::new(Iron {}),
        Box::new(Cobalt {}),
        Box::new(Nickel {}),
        Box::new(Copper {}),
        Box::new(Zinc {}),
        Box::new(Gallium {}),
        Box::new(Germanium {}),
        Box::new(Arsenic {}),
        Box::new(Selenium {}),
        Box::new(Bromium {}),
        Box::new(Krypton {}),
        Box::new(Rubidium {}),
        Box::new(Strontium {}),
        Box::new(Yttrium {}),
        Box::new(Zirconium {}),
        Box::new(Niobium {}),
        Box::new(Molybdenum {}),
        Box::new(Technetium {}),
        Box::new(Ruthenium {}),
        Box::new(Rhodium {}),
        Box::new(Palladium {}),
        Box::new(Silver {}),
        Box::new(Cadmium {}),
        Box::new(Indium {}),
        Box::new(Tin {}),
        Box::new(Antimony {}),
        Box::new(Tellurium {}),
        Box::new(Iodine {}),
        Box::new(Xenon {}),
        Box::new(Cesium {}),
        Box::new(Barium {}),
        Box::new(Lanthanum {}),
        Box::new(Cerium {}),
        Box::new(Prazeodymium {}),
        Box::new(Neodymium {}),
        Box::new(Promethium {}),
        Box::new(Sammarium {}),
        Box::new(Europium {}),
        Box::new(Gadolium {}),
        Box::new(Terbium {}),
        Box::new(Dysprosium {}),
        Box::new(Holmium {}),
        Box::new(Erbium {}),
        Box::new(Thulium {}),
        Box::new(Ytterbium {}),
        Box::new(Lutetium {}),
        Box::new(Hafnium {}),
        Box::new(Tantalum {}),
        Box::new(Tungsten {}),
        Box::new(Rhenium {}),
        Box::new(Osmium {}),
        Box::new(Ir {}),
        Box::new(Platinum {}),
        Box::new(Gold {}),
        Box::new(Mercury {}),
        Box::new(Thallium {}),
        Box::new(Lead {}),
        Box::new(Bismuth {}),
        Box::new(Polonium {}),
        Box::new(Astatine {}),
        Box::new(Radon {}),
        Box::new(Fr {}),
        Box::new(Radium {}),
        Box::new(Actinium {}),
        Box::new(Thorium {}),
        Box::new(Protactinium {}),
        Box::new(Uranium {}),
        Box::new(Neptunium {}),
        Box::new(Plutonium {}),
        Box::new(Americium {}),
        Box::new(Curium {}),
        Box::new(Berkelium {}),
        Box::new(Californium {}),
        Box::new(Einstein {}),
        Box::new(Fermium {}),
        Box::new(Mendelevium {}),
        Box::new(Nobelium {}),
        Box::new(Larensium {}),
        Box::new(Rutherford {}),
        Box::new(Dubnium {}),
        Box::new(Seaborgium {}),
        Box::new(Bohr {}),
        Box::new(Hassium {}),
        Box::new(Meitnerium {}),
        Box::new(Darmstad {}),
        Box::new(Roentgenium {}),
        Box::new(Copernicus {}),
        Box::new(Nihon {}),
        Box::new(Flerovium {}),
        Box::new(Moscovium {}),
        Box::new(Livermorium {}),
        Box::new(Tenes {}),
        Box::new(Oganesson {}),
    ]
}

fn print_chemical_element_static<T: ChemicalElement>(element: T) {
    println!("{}: {}", element.short_name(), element.atomic_number());
}

fn print_chemical_element_dynamic(element: &dyn ChemicalElement) {
    println!("{}: {}", element.short_name(), element.atomic_number());
}

fn main() {
    let elements = instantiate_elements();

    let start = Instant::now();


    // for element in elements.iter() {
    //     print_chemical_element_dynamic(element.as_ref());
    // }

    // print_chemical_element_static(Hydrogen {});
    // print_chemical_element_static(Helium {});
    // print_chemical_element_static(Lithium {});
    // print_chemical_element_static(Beryllium {});
    // print_chemical_element_static(Boron {});
    // print_chemical_element_static(Carbon {});
    // print_chemical_element_static(Nitrogen {});
    // print_chemical_element_static(Oxygen {});
    // print_chemical_element_static(Fluorine {});
    // print_chemical_element_static(Neon {});
    // print_chemical_element_static(Sodium {});
    // print_chemical_element_static(Magnesium {});
    // print_chemical_element_static(Aluminum {});
    // print_chemical_element_static(Silicon {});
    // print_chemical_element_static(Phosphorus {});
    // print_chemical_element_static(Sulfur {});
    // print_chemical_element_static(Chlorine {});
    // print_chemical_element_static(Argon {});
    // print_chemical_element_static(Potassium {});
    // print_chemical_element_static(Calcium {});
    // print_chemical_element_static(Scandium {});
    // print_chemical_element_static(Titanium {});
    // print_chemical_element_static(Vanadium {});
    // print_chemical_element_static(Chromium {});
    // print_chemical_element_static(Manganese {});
    // print_chemical_element_static(Iron {});
    // print_chemical_element_static(Cobalt {});
    // print_chemical_element_static(Nickel {});
    // print_chemical_element_static(Copper {});
    // print_chemical_element_static(Zinc {});
    // print_chemical_element_static(Gallium {});
    // print_chemical_element_static(Germanium {});
    // print_chemical_element_static(Arsenic {});
    // print_chemical_element_static(Selenium {});
    // print_chemical_element_static(Bromium {});
    // print_chemical_element_static(Krypton {});
    // print_chemical_element_static(Rubidium {});
    // print_chemical_element_static(Strontium {});
    // print_chemical_element_static(Yttrium {});
    // print_chemical_element_static(Zirconium {});
    // print_chemical_element_static(Niobium {});
    // print_chemical_element_static(Molybdenum {});
    // print_chemical_element_static(Technetium {});
    // print_chemical_element_static(Ruthenium {});
    // print_chemical_element_static(Rhodium {});
    // print_chemical_element_static(Palladium {});
    // print_chemical_element_static(Silver {});
    // print_chemical_element_static(Cadmium {});
    // print_chemical_element_static(Indium {});
    // print_chemical_element_static(Tin {});
    // print_chemical_element_static(Antimony {});
    // print_chemical_element_static(Tellurium {});
    // print_chemical_element_static(Iodine {});
    // print_chemical_element_static(Xenon {});
    // print_chemical_element_static(Cesium {});
    // print_chemical_element_static(Barium {});
    // print_chemical_element_static(Lanthanum {});
    // print_chemical_element_static(Cerium {});
    // print_chemical_element_static(Prazeodymium {});
    // print_chemical_element_static(Neodymium {});
    // print_chemical_element_static(Promethium {});
    // print_chemical_element_static(Sammarium {});
    // print_chemical_element_static(Europium {});
    // print_chemical_element_static(Gadolium {});
    // print_chemical_element_static(Terbium {});
    // print_chemical_element_static(Dysprosium {});
    // print_chemical_element_static(Holmium {});
    // print_chemical_element_static(Erbium {});
    // print_chemical_element_static(Thulium {});
    // print_chemical_element_static(Ytterbium {});
    // print_chemical_element_static(Lutetium {});
    // print_chemical_element_static(Hafnium {});
    // print_chemical_element_static(Tantalum {});
    // print_chemical_element_static(Tungsten {});
    // print_chemical_element_static(Rhenium {});
    // print_chemical_element_static(Osmium {});
    // print_chemical_element_static(Ir {});
    // print_chemical_element_static(Platinum {});
    // print_chemical_element_static(Gold {});
    // print_chemical_element_static(Mercury {});
    // print_chemical_element_static(Thallium {});
    // print_chemical_element_static(Lead {});
    // print_chemical_element_static(Bismuth {});
    // print_chemical_element_static(Polonium {});
    // print_chemical_element_static(Astatine {});
    // print_chemical_element_static(Radon {});
    // print_chemical_element_static(Fr {});
    // print_chemical_element_static(Radium {});
    // print_chemical_element_static(Actinium {});
    // print_chemical_element_static(Thorium {});
    // print_chemical_element_static(Protactinium {});
    // print_chemical_element_static(Uranium {});
    // print_chemical_element_static(Neptunium {});
    // print_chemical_element_static(Plutonium {});
    // print_chemical_element_static(Americium {});
    // print_chemical_element_static(Curium {});
    // print_chemical_element_static(Berkelium {});
    // print_chemical_element_static(Californium {});
    // print_chemical_element_static(Einstein {});
    // print_chemical_element_static(Fermium {});
    // print_chemical_element_static(Mendelevium {});
    // print_chemical_element_static(Nobelium {});
    // print_chemical_element_static(Larensium {});
    // print_chemical_element_static(Rutherford {});
    // print_chemical_element_static(Dubnium {});
    // print_chemical_element_static(Seaborgium {});
    // print_chemical_element_static(Bohr {});
    // print_chemical_element_static(Hassium {});
    // print_chemical_element_static(Meitnerium {});
    // print_chemical_element_static(Darmstad {});
    // print_chemical_element_static(Roentgenium {});
    // print_chemical_element_static(Copernicus {});
    // print_chemical_element_static(Nihon {});
    // print_chemical_element_static(Flerovium {});
    // print_chemical_element_static(Moscovium {});
    // print_chemical_element_static(Livermorium {});
    // print_chemical_element_static(Tenes {});
    // print_chemical_element_static(Oganesson {});

    println!("Time elapsed: {:?}", start.elapsed());
}




