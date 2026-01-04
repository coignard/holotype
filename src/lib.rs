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

pub mod cli;
pub mod config;
pub mod data;
pub mod decoder;
pub mod formatter;
pub mod generator;
pub mod pronounceability;
pub mod phonotactics;

use chrono::{Datelike, Local, NaiveDate};
use cli::Cli;
use config::Config;
use data::Morphemes;
use formatter::DecodedName;

pub fn run(cli: Cli) -> Result<(), String> {
    let morphemes = Morphemes::new();
    let config = Config::default();
    config.validate()?;

    let salt = cli.salt.as_deref().unwrap_or("");

    if cli.extract {
        let name = cli.value.ok_or("Name required for extraction")?;

        match decoder::decode(&name, salt, &morphemes, &config) {
            Some((date, number)) => {
                let decoded = DecodedName::new(name, date, number, salt);
                decoded.display();
                Ok(())
            }
            None => Err(format!("Could not decode name: {}", name)),
        }
    } else {
        let number = if let Some(idx) = cli.index {
            idx
        } else if let Some(val) = cli.value {
            val.parse::<u32>()
                .map_err(|_| format!("Invalid number: {}", val))?
        } else {
            return Err("Number required (provide as argument or use -i)".to_string());
        };

        if !(config.number_min..=config.number_max).contains(&number) {
            return Err(format!(
                "Number {} is out of range [{}, {}]",
                number, config.number_min, config.number_max
            ));
        }

        let date = if let Some(date_str) = cli.date {
            NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(|_| format!("Invalid date format: {}", date_str))?
        } else {
            Local::now().date_naive()
        };

        if date.year() < config.year_start || date.year() > config.year_end {
            return Err(format!(
                "Date year {} is out of range [{}, {}]",
                date.year(),
                config.year_start,
                config.year_end
            ));
        }

        let name = generator::generate_name(date, number, salt, &morphemes, &config);
        println!("{}", name);
        Ok(())
    }
}
