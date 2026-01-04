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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Origin {
    Greek,
    Latin,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Category {
    Size,
    Colour,
    Position,
    Time,
    Number,
    Form,
    Environment,
    Quality,
}

#[derive(Copy, Clone, Debug)]
pub struct Morpheme {
    pub text: &'static str,
    pub origin: Origin,
    pub category: Category,
}

pub struct Morphemes {
    pub prefixes: &'static [Morpheme],
    pub roots: &'static [&'static str],
    pub genus_suffixes: &'static [&'static str],
    pub species_descriptors: &'static [SpeciesDescriptor],
}

#[derive(Copy, Clone, Debug)]
pub struct SpeciesDescriptor {
    pub text: &'static str,
    pub category: Option<Category>,
}

impl Morphemes {
    pub fn new() -> Self {
        Self {
            prefixes: super::PREFIXES,
            roots: super::ROOTS,
            genus_suffixes: super::GENUS_SUFFIXES,
            species_descriptors: super::SPECIES_DESCRIPTORS,
        }
    }
}

impl Default for Morphemes {
    fn default() -> Self {
        Self::new()
    }
}
