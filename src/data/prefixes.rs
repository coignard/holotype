// This file is part of Holotype.
//
// Copyright (c) 2026  René Coignard <contact@renecoignard.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use super::morphemes::{Category, Morpheme, Origin};

pub const PREFIXES: &[Morpheme] = &[
    Morpheme {
        text: "Macro",
        origin: Origin::Greek,
        category: Category::Size,
    },
    Morpheme {
        text: "Micro",
        origin: Origin::Greek,
        category: Category::Size,
    },
    Morpheme {
        text: "Mega",
        origin: Origin::Greek,
        category: Category::Size,
    },
    Morpheme {
        text: "Mini",
        origin: Origin::Latin,
        category: Category::Size,
    },
    Morpheme {
        text: "Magni",
        origin: Origin::Latin,
        category: Category::Size,
    },
    Morpheme {
        text: "Parvi",
        origin: Origin::Latin,
        category: Category::Size,
    },
    Morpheme {
        text: "Maxi",
        origin: Origin::Latin,
        category: Category::Size,
    },
    Morpheme {
        text: "Grandi",
        origin: Origin::Latin,
        category: Category::Size,
    },
    Morpheme {
        text: "Brachy",
        origin: Origin::Greek,
        category: Category::Size,
    },
    Morpheme {
        text: "Lepto",
        origin: Origin::Greek,
        category: Category::Size,
    },
    Morpheme {
        text: "Longi",
        origin: Origin::Latin,
        category: Category::Size,
    },
    Morpheme {
        text: "Brevi",
        origin: Origin::Latin,
        category: Category::Size,
    },
    Morpheme {
        text: "Lati",
        origin: Origin::Latin,
        category: Category::Size,
    },
    Morpheme {
        text: "Angusti",
        origin: Origin::Latin,
        category: Category::Size,
    },
    Morpheme {
        text: "Alti",
        origin: Origin::Latin,
        category: Category::Size,
    },
    Morpheme {
        text: "Bathy",
        origin: Origin::Greek,
        category: Category::Size,
    },
    Morpheme {
        text: "Leuco",
        origin: Origin::Greek,
        category: Category::Colour,
    },
    Morpheme {
        text: "Melano",
        origin: Origin::Greek,
        category: Category::Colour,
    },
    Morpheme {
        text: "Xantho",
        origin: Origin::Greek,
        category: Category::Colour,
    },
    Morpheme {
        text: "Chloro",
        origin: Origin::Greek,
        category: Category::Colour,
    },
    Morpheme {
        text: "Rhodo",
        origin: Origin::Greek,
        category: Category::Colour,
    },
    Morpheme {
        text: "Cyano",
        origin: Origin::Greek,
        category: Category::Colour,
    },
    Morpheme {
        text: "Porphyro",
        origin: Origin::Greek,
        category: Category::Colour,
    },
    Morpheme {
        text: "Albo",
        origin: Origin::Latin,
        category: Category::Colour,
    },
    Morpheme {
        text: "Nigri",
        origin: Origin::Latin,
        category: Category::Colour,
    },
    Morpheme {
        text: "Rubi",
        origin: Origin::Latin,
        category: Category::Colour,
    },
    Morpheme {
        text: "Flavi",
        origin: Origin::Latin,
        category: Category::Colour,
    },
    Morpheme {
        text: "Fulvi",
        origin: Origin::Latin,
        category: Category::Colour,
    },
    Morpheme {
        text: "Griseo",
        origin: Origin::Latin,
        category: Category::Colour,
    },
    Morpheme {
        text: "Roseo",
        origin: Origin::Latin,
        category: Category::Colour,
    },
    Morpheme {
        text: "Luteo",
        origin: Origin::Latin,
        category: Category::Colour,
    },
    Morpheme {
        text: "Argenti",
        origin: Origin::Latin,
        category: Category::Colour,
    },
    Morpheme {
        text: "Auri",
        origin: Origin::Latin,
        category: Category::Colour,
    },
    Morpheme {
        text: "Ferru",
        origin: Origin::Latin,
        category: Category::Colour,
    },
    Morpheme {
        text: "Hydro",
        origin: Origin::Greek,
        category: Category::Environment,
    },
    Morpheme {
        text: "Pyro",
        origin: Origin::Greek,
        category: Category::Environment,
    },
    Morpheme {
        text: "Cryo",
        origin: Origin::Greek,
        category: Category::Environment,
    },
    Morpheme {
        text: "Geo",
        origin: Origin::Greek,
        category: Category::Environment,
    },
    Morpheme {
        text: "Aero",
        origin: Origin::Greek,
        category: Category::Environment,
    },
    Morpheme {
        text: "Litho",
        origin: Origin::Greek,
        category: Category::Environment,
    },
    Morpheme {
        text: "Thermo",
        origin: Origin::Greek,
        category: Category::Environment,
    },
    Morpheme {
        text: "Photo",
        origin: Origin::Greek,
        category: Category::Environment,
    },
    Morpheme {
        text: "Hygro",
        origin: Origin::Greek,
        category: Category::Environment,
    },
    Morpheme {
        text: "Xero",
        origin: Origin::Greek,
        category: Category::Environment,
    },
    Morpheme {
        text: "Halo",
        origin: Origin::Greek,
        category: Category::Environment,
    },
    Morpheme {
        text: "Psammo",
        origin: Origin::Greek,
        category: Category::Environment,
    },
    Morpheme {
        text: "Aqu",
        origin: Origin::Latin,
        category: Category::Environment,
    },
    Morpheme {
        text: "Mari",
        origin: Origin::Latin,
        category: Category::Environment,
    },
    Morpheme {
        text: "Monti",
        origin: Origin::Latin,
        category: Category::Environment,
    },
    Morpheme {
        text: "Silvi",
        origin: Origin::Latin,
        category: Category::Environment,
    },
    Morpheme {
        text: "Glaci",
        origin: Origin::Latin,
        category: Category::Environment,
    },
    Morpheme {
        text: "Petri",
        origin: Origin::Latin,
        category: Category::Environment,
    },
    Morpheme {
        text: "Litori",
        origin: Origin::Latin,
        category: Category::Environment,
    },
    Morpheme {
        text: "Nivi",
        origin: Origin::Latin,
        category: Category::Environment,
    },
    Morpheme {
        text: "Neo",
        origin: Origin::Greek,
        category: Category::Time,
    },
    Morpheme {
        text: "Paleo",
        origin: Origin::Greek,
        category: Category::Time,
    },
    Morpheme {
        text: "Archaeo",
        origin: Origin::Greek,
        category: Category::Time,
    },
    Morpheme {
        text: "Chrono",
        origin: Origin::Greek,
        category: Category::Time,
    },
    Morpheme {
        text: "Proto",
        origin: Origin::Greek,
        category: Category::Time,
    },
    Morpheme {
        text: "Eo",
        origin: Origin::Greek,
        category: Category::Time,
    },
    Morpheme {
        text: "Meso",
        origin: Origin::Greek,
        category: Category::Time,
    },
    Morpheme {
        text: "Ceno",
        origin: Origin::Greek,
        category: Category::Time,
    },
    Morpheme {
        text: "Novi",
        origin: Origin::Latin,
        category: Category::Time,
    },
    Morpheme {
        text: "Anti",
        origin: Origin::Latin,
        category: Category::Time,
    },
    Morpheme {
        text: "Primi",
        origin: Origin::Latin,
        category: Category::Time,
    },
    Morpheme {
        text: "Endo",
        origin: Origin::Greek,
        category: Category::Position,
    },
    Morpheme {
        text: "Ecto",
        origin: Origin::Greek,
        category: Category::Position,
    },
    Morpheme {
        text: "Epi",
        origin: Origin::Greek,
        category: Category::Position,
    },
    Morpheme {
        text: "Hypo",
        origin: Origin::Greek,
        category: Category::Position,
    },
    Morpheme {
        text: "Hyper",
        origin: Origin::Greek,
        category: Category::Position,
    },
    Morpheme {
        text: "Peri",
        origin: Origin::Greek,
        category: Category::Position,
    },
    Morpheme {
        text: "Para",
        origin: Origin::Greek,
        category: Category::Position,
    },
    Morpheme {
        text: "Meta",
        origin: Origin::Greek,
        category: Category::Position,
    },
    Morpheme {
        text: "Ana",
        origin: Origin::Greek,
        category: Category::Position,
    },
    Morpheme {
        text: "Cata",
        origin: Origin::Greek,
        category: Category::Position,
    },
    Morpheme {
        text: "Amphi",
        origin: Origin::Greek,
        category: Category::Position,
    },
    Morpheme {
        text: "Super",
        origin: Origin::Latin,
        category: Category::Position,
    },
    Morpheme {
        text: "Sub",
        origin: Origin::Latin,
        category: Category::Position,
    },
    Morpheme {
        text: "Trans",
        origin: Origin::Latin,
        category: Category::Position,
    },
    Morpheme {
        text: "Inter",
        origin: Origin::Latin,
        category: Category::Position,
    },
    Morpheme {
        text: "Infra",
        origin: Origin::Latin,
        category: Category::Position,
    },
    Morpheme {
        text: "Ultra",
        origin: Origin::Latin,
        category: Category::Position,
    },
    Morpheme {
        text: "Circum",
        origin: Origin::Latin,
        category: Category::Position,
    },
    Morpheme {
        text: "Mono",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Di",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Tri",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Tetra",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Penta",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Hexa",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Hepta",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Octo",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Ennea",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Deca",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Poly",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Oligo",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Diplo",
        origin: Origin::Greek,
        category: Category::Number,
    },
    Morpheme {
        text: "Uni",
        origin: Origin::Latin,
        category: Category::Number,
    },
    Morpheme {
        text: "Bi",
        origin: Origin::Latin,
        category: Category::Number,
    },
    Morpheme {
        text: "Quadri",
        origin: Origin::Latin,
        category: Category::Number,
    },
    Morpheme {
        text: "Multi",
        origin: Origin::Latin,
        category: Category::Number,
    },
    Morpheme {
        text: "Pluri",
        origin: Origin::Latin,
        category: Category::Number,
    },
    Morpheme {
        text: "Semi",
        origin: Origin::Latin,
        category: Category::Number,
    },
    Morpheme {
        text: "Pauci",
        origin: Origin::Latin,
        category: Category::Number,
    },
    Morpheme {
        text: "Morpho",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Platy",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Strepto",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Cyclo",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Spheno",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Sphaero",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Sclero",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Trachy",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Lopho",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Ortho",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Schizo",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Holo",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Stereo",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Stylo",
        origin: Origin::Greek,
        category: Category::Form,
    },
    Morpheme {
        text: "Plani",
        origin: Origin::Latin,
        category: Category::Form,
    },
    Morpheme {
        text: "Curvi",
        origin: Origin::Latin,
        category: Category::Form,
    },
    Morpheme {
        text: "Recti",
        origin: Origin::Latin,
        category: Category::Form,
    },
    Morpheme {
        text: "Spiri",
        origin: Origin::Latin,
        category: Category::Form,
    },
    Morpheme {
        text: "Globi",
        origin: Origin::Latin,
        category: Category::Form,
    },
    Morpheme {
        text: "Squami",
        origin: Origin::Latin,
        category: Category::Form,
    },
    Morpheme {
        text: "Stelli",
        origin: Origin::Latin,
        category: Category::Form,
    },
    Morpheme {
        text: "Rhombi",
        origin: Origin::Latin,
        category: Category::Form,
    },
    Morpheme {
        text: "Crypto",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Pseudo",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Eu",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Hetero",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Homo",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Iso",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Aniso",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Allo",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Auto",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Syn",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Apo",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Gymno",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Hapto",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Acantho",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Actino",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Tachy",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Brady",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Steno",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Eury",
        origin: Origin::Greek,
        category: Category::Quality,
    },
    Morpheme {
        text: "Simpli",
        origin: Origin::Latin,
        category: Category::Quality,
    },
    Morpheme {
        text: "Vari",
        origin: Origin::Latin,
        category: Category::Quality,
    },
    Morpheme {
        text: "Vermi",
        origin: Origin::Latin,
        category: Category::Quality,
    },
    Morpheme {
        text: "Serri",
        origin: Origin::Latin,
        category: Category::Quality,
    },
    Morpheme {
        text: "Spini",
        origin: Origin::Latin,
        category: Category::Quality,
    },
    Morpheme {
        text: "Totu",
        origin: Origin::Latin,
        category: Category::Quality,
    },
];

const fn check_uniqueness() {
    let mut i = 0;
    while i < PREFIXES.len() {
        let mut j = i + 1;
        while j < PREFIXES.len() {
            let text_i = PREFIXES[i].text.as_bytes();
            let text_j = PREFIXES[j].text.as_bytes();

            if text_i.len() == text_j.len() {
                let mut k = 0;
                let mut all_match = true;
                while k < text_i.len() {
                    if text_i[k] != text_j[k] {
                        all_match = false;
                        break;
                    }
                    k += 1;
                }

                if all_match {
                    panic!("Duplicate prefix found");
                }
            }
            j += 1;
        }
        i += 1;
    }
}

const _: () = check_uniqueness();

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_no_duplicates() {
        let mut seen = HashSet::new();
        for morpheme in PREFIXES {
            assert!(
                seen.insert(morpheme.text),
                "Duplicate prefix: {}",
                morpheme.text
            );
        }
    }
}
