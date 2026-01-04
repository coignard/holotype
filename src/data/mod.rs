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

mod morphemes;
mod prefixes;
mod roots;
mod suffixes;

pub use morphemes::{Category, Morpheme, Morphemes, Origin, SpeciesDescriptor};
pub use prefixes::PREFIXES;
pub use roots::ROOTS;
pub use suffixes::{GENUS_SUFFIXES, SPECIES_DESCRIPTORS};
