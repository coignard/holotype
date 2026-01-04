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

pub struct Config {
    pub year_start: i32,
    pub year_end: i32,
    pub number_min: u32,
    pub number_max: u32,
    pub max_consonant_cluster: usize,
    pub min_pronounceability_score: f32,
    pub max_genus_length: usize,
}

impl Config {
    pub const fn default() -> Self {
        Self {
            year_start: 2000,
            year_end: 2099,
            number_min: 1,
            number_max: 99,
            max_consonant_cluster: 3,
            min_pronounceability_score: 0.3,
            max_genus_length: 18,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.year_start >= self.year_end {
            return Err("year_start must be less than year_end".to_string());
        }
        if self.number_min >= self.number_max {
            return Err("number_min must be less than number_max".to_string());
        }
        if self.max_consonant_cluster < 2 {
            return Err("max_consonant_cluster must be at least 2".to_string());
        }
        if !(0.0..=1.0).contains(&self.min_pronounceability_score) {
            return Err("min_pronounceability_score must be between 0.0 and 1.0".to_string());
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::default()
    }
}
