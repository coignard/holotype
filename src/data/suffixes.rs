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

pub const GENUS_SUFFIXES: &[&str] = &[
    "us", "os", "es", "is", "a", "e", "as", "um", "on", "ma",
    "er", "or", "en", "yx", "ix", "ax",
];

use super::morphemes::{Category, SpeciesDescriptor};

pub const SPECIES_DESCRIPTORS: &[SpeciesDescriptor] = &[
    SpeciesDescriptor {
        text: "robustus",
        category: Some(Category::Size),
    },
    SpeciesDescriptor {
        text: "validus",
        category: Some(Category::Size),
    },
    SpeciesDescriptor {
        text: "gracilis",
        category: Some(Category::Size),
    },
    SpeciesDescriptor {
        text: "tenuis",
        category: Some(Category::Size),
    },
    SpeciesDescriptor {
        text: "crassus",
        category: Some(Category::Size),
    },
    SpeciesDescriptor {
        text: "densus",
        category: Some(Category::Size),
    },
    SpeciesDescriptor {
        text: "solidus",
        category: Some(Category::Size),
    },
    SpeciesDescriptor {
        text: "pinguis",
        category: Some(Category::Size),
    },
    SpeciesDescriptor {
        text: "obesus",
        category: Some(Category::Size),
    },
    SpeciesDescriptor {
        text: "macilentus",
        category: Some(Category::Size),
    },
    SpeciesDescriptor {
        text: "pallidus",
        category: Some(Category::Colour),
    },
    SpeciesDescriptor {
        text: "obscurus",
        category: Some(Category::Colour),
    },
    SpeciesDescriptor {
        text: "lucidus",
        category: Some(Category::Colour),
    },
    SpeciesDescriptor {
        text: "nitidus",
        category: Some(Category::Colour),
    },
    SpeciesDescriptor {
        text: "opacus",
        category: Some(Category::Colour),
    },
    SpeciesDescriptor {
        text: "maculatus",
        category: Some(Category::Colour),
    },
    SpeciesDescriptor {
        text: "striatus",
        category: Some(Category::Colour),
    },
    SpeciesDescriptor {
        text: "variegatus",
        category: Some(Category::Colour),
    },
    SpeciesDescriptor {
        text: "pictus",
        category: Some(Category::Colour),
    },
    SpeciesDescriptor {
        text: "tinctus",
        category: Some(Category::Colour),
    },
    SpeciesDescriptor {
        text: "humidus",
        category: Some(Category::Environment),
    },
    SpeciesDescriptor {
        text: "siccus",
        category: Some(Category::Environment),
    },
    SpeciesDescriptor {
        text: "frigidus",
        category: Some(Category::Environment),
    },
    SpeciesDescriptor {
        text: "calidus",
        category: Some(Category::Environment),
    },
    SpeciesDescriptor {
        text: "umbratus",
        category: Some(Category::Environment),
    },
    SpeciesDescriptor {
        text: "apricus",
        category: Some(Category::Environment),
    },
    SpeciesDescriptor {
        text: "ventosus",
        category: Some(Category::Environment),
    },
    SpeciesDescriptor {
        text: "pluvialis",
        category: Some(Category::Environment),
    },
    SpeciesDescriptor {
        text: "nivalis",
        category: Some(Category::Environment),
    },
    SpeciesDescriptor {
        text: "rupicola",
        category: Some(Category::Environment),
    },
    SpeciesDescriptor {
        text: "temporalis",
        category: Some(Category::Time),
    },
    SpeciesDescriptor {
        text: "aeternus",
        category: Some(Category::Time),
    },
    SpeciesDescriptor {
        text: "diurnus",
        category: Some(Category::Time),
    },
    SpeciesDescriptor {
        text: "nocturnus",
        category: Some(Category::Time),
    },
    SpeciesDescriptor {
        text: "matutinus",
        category: Some(Category::Time),
    },
    SpeciesDescriptor {
        text: "vespertinus",
        category: Some(Category::Time),
    },
    SpeciesDescriptor {
        text: "vernalis",
        category: Some(Category::Time),
    },
    SpeciesDescriptor {
        text: "aestivus",
        category: Some(Category::Time),
    },
    SpeciesDescriptor {
        text: "autumnalis",
        category: Some(Category::Time),
    },
    SpeciesDescriptor {
        text: "hiemalis",
        category: Some(Category::Time),
    },
    SpeciesDescriptor {
        text: "medianus",
        category: Some(Category::Position),
    },
    SpeciesDescriptor {
        text: "lateralis",
        category: Some(Category::Position),
    },
    SpeciesDescriptor {
        text: "centralis",
        category: Some(Category::Position),
    },
    SpeciesDescriptor {
        text: "periphericus",
        category: Some(Category::Position),
    },
    SpeciesDescriptor {
        text: "extremus",
        category: Some(Category::Position),
    },
    SpeciesDescriptor {
        text: "medius",
        category: Some(Category::Position),
    },
    SpeciesDescriptor {
        text: "imus",
        category: Some(Category::Position),
    },
    SpeciesDescriptor {
        text: "summus",
        category: Some(Category::Position),
    },
    SpeciesDescriptor {
        text: "aggregatus",
        category: Some(Category::Number),
    },
    SpeciesDescriptor {
        text: "dispersus",
        category: Some(Category::Number),
    },
    SpeciesDescriptor {
        text: "confertus",
        category: Some(Category::Number),
    },
    SpeciesDescriptor {
        text: "sparsus",
        category: Some(Category::Number),
    },
    SpeciesDescriptor {
        text: "copiousus",
        category: Some(Category::Number),
    },
    SpeciesDescriptor {
        text: "solitarius",
        category: Some(Category::Number),
    },
    SpeciesDescriptor {
        text: "gregarius",
        category: Some(Category::Number),
    },
    SpeciesDescriptor {
        text: "colonialis",
        category: Some(Category::Number),
    },
    SpeciesDescriptor {
        text: "regularis",
        category: Some(Category::Form),
    },
    SpeciesDescriptor {
        text: "irregularis",
        category: Some(Category::Form),
    },
    SpeciesDescriptor {
        text: "symmetricus",
        category: Some(Category::Form),
    },
    SpeciesDescriptor {
        text: "asymmetricus",
        category: Some(Category::Form),
    },
    SpeciesDescriptor {
        text: "compressus",
        category: Some(Category::Form),
    },
    SpeciesDescriptor {
        text: "depressus",
        category: Some(Category::Form),
    },
    SpeciesDescriptor {
        text: "inflatus",
        category: Some(Category::Form),
    },
    SpeciesDescriptor {
        text: "contortus",
        category: Some(Category::Form),
    },
    SpeciesDescriptor {
        text: "flexuosus",
        category: Some(Category::Form),
    },
    SpeciesDescriptor {
        text: "undulatus",
        category: Some(Category::Form),
    },
    SpeciesDescriptor {
        text: "perfectus",
        category: Some(Category::Quality),
    },
    SpeciesDescriptor {
        text: "imperfectus",
        category: Some(Category::Quality),
    },
    SpeciesDescriptor {
        text: "completus",
        category: Some(Category::Quality),
    },
    SpeciesDescriptor {
        text: "incompletus",
        category: Some(Category::Quality),
    },
    SpeciesDescriptor {
        text: "verus",
        category: Some(Category::Quality),
    },
    SpeciesDescriptor {
        text: "falsus",
        category: Some(Category::Quality),
    },
    SpeciesDescriptor {
        text: "spurius",
        category: Some(Category::Quality),
    },
    SpeciesDescriptor {
        text: "hybridus",
        category: Some(Category::Quality),
    },
    SpeciesDescriptor {
        text: "alpinus",
        category: None,
    },
    SpeciesDescriptor {
        text: "maritimus",
        category: None,
    },
    SpeciesDescriptor {
        text: "montanus",
        category: None,
    },
    SpeciesDescriptor {
        text: "campestris",
        category: None,
    },
    SpeciesDescriptor {
        text: "sylvaticus",
        category: None,
    },
    SpeciesDescriptor {
        text: "urbanus",
        category: None,
    },
    SpeciesDescriptor {
        text: "borealis",
        category: None,
    },
    SpeciesDescriptor {
        text: "australis",
        category: None,
    },
    SpeciesDescriptor {
        text: "orientalis",
        category: None,
    },
    SpeciesDescriptor {
        text: "occidentalis",
        category: None,
    },
    SpeciesDescriptor {
        text: "insularis",
        category: None,
    },
    SpeciesDescriptor {
        text: "rupestris",
        category: None,
    },
    SpeciesDescriptor {
        text: "pratensis",
        category: None,
    },
    SpeciesDescriptor {
        text: "paludosus",
        category: None,
    },
    SpeciesDescriptor {
        text: "lacustris",
        category: None,
    },
    SpeciesDescriptor {
        text: "fluvialis",
        category: None,
    },
    SpeciesDescriptor {
        text: "riparius",
        category: None,
    },
    SpeciesDescriptor {
        text: "terrestris",
        category: None,
    },
    SpeciesDescriptor {
        text: "arenarius",
        category: None,
    },
    SpeciesDescriptor {
        text: "saxatilis",
        category: None,
    },
    SpeciesDescriptor {
        text: "elegans",
        category: None,
    },
    SpeciesDescriptor {
        text: "formosus",
        category: None,
    },
    SpeciesDescriptor {
        text: "pulcher",
        category: None,
    },
    SpeciesDescriptor {
        text: "ornatus",
        category: None,
    },
    SpeciesDescriptor {
        text: "decorus",
        category: None,
    },
    SpeciesDescriptor {
        text: "venustus",
        category: None,
    },
    SpeciesDescriptor {
        text: "spectabilis",
        category: None,
    },
    SpeciesDescriptor {
        text: "insignis",
        category: None,
    },
    SpeciesDescriptor {
        text: "eximius",
        category: None,
    },
    SpeciesDescriptor {
        text: "admirabilis",
        category: None,
    },
    SpeciesDescriptor {
        text: "mirabilis",
        category: None,
    },
    SpeciesDescriptor {
        text: "horridus",
        category: None,
    },
    SpeciesDescriptor {
        text: "deformis",
        category: None,
    },
    SpeciesDescriptor {
        text: "monstrosus",
        category: None,
    },
    SpeciesDescriptor {
        text: "velox",
        category: None,
    },
    SpeciesDescriptor {
        text: "agilis",
        category: None,
    },
    SpeciesDescriptor {
        text: "tardus",
        category: None,
    },
    SpeciesDescriptor {
        text: "quietus",
        category: None,
    },
    SpeciesDescriptor {
        text: "errans",
        category: None,
    },
    SpeciesDescriptor {
        text: "vagans",
        category: None,
    },
    SpeciesDescriptor {
        text: "migrans",
        category: None,
    },
    SpeciesDescriptor {
        text: "sedentarius",
        category: None,
    },
    SpeciesDescriptor {
        text: "pugnax",
        category: None,
    },
    SpeciesDescriptor {
        text: "timidus",
        category: None,
    },
    SpeciesDescriptor {
        text: "audax",
        category: None,
    },
    SpeciesDescriptor {
        text: "ferox",
        category: None,
    },
    SpeciesDescriptor {
        text: "vulgaris",
        category: None,
    },
    SpeciesDescriptor {
        text: "communis",
        category: None,
    },
    SpeciesDescriptor {
        text: "rarus",
        category: None,
    },
    SpeciesDescriptor {
        text: "frequens",
        category: None,
    },
    SpeciesDescriptor {
        text: "abundans",
        category: None,
    },
    SpeciesDescriptor {
        text: "parasiticus",
        category: None,
    },
    SpeciesDescriptor {
        text: "symbioticus",
        category: None,
    },
    SpeciesDescriptor {
        text: "saprophyticus",
        category: None,
    },
    SpeciesDescriptor {
        text: "epiphyticus",
        category: None,
    },
    SpeciesDescriptor {
        text: "domesticus",
        category: None,
    },
    SpeciesDescriptor {
        text: "ferus",
        category: None,
    },
    SpeciesDescriptor {
        text: "cultivatus",
        category: None,
    },
    SpeciesDescriptor {
        text: "major",
        category: None,
    },
    SpeciesDescriptor {
        text: "minor",
        category: None,
    },
    SpeciesDescriptor {
        text: "medius",
        category: None,
    },
    SpeciesDescriptor {
        text: "paradoxus",
        category: None,
    },
    SpeciesDescriptor {
        text: "insolitus",
        category: None,
    },
    SpeciesDescriptor {
        text: "curiosus",
        category: None,
    },
    SpeciesDescriptor {
        text: "dubius",
        category: None,
    },
    SpeciesDescriptor {
        text: "ambiguus",
        category: None,
    },
    SpeciesDescriptor {
        text: "incertus",
        category: None,
    },
];
