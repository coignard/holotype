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

use clap::Parser;

#[derive(Parser)]
#[command(name = "holotype")]
#[command(about = "Generate reproducible names for musical projects and patches")]
pub struct Cli {
    /// Ordinal number for generation (or name for extraction with -x)
    pub value: Option<String>,

    /// Index
    #[arg(short, long, value_name = "NUMBER")]
    pub index: Option<u32>,

    /// Date (YYYY-MM-DD format, defaults to today)
    #[arg(short, long, value_name = "DATE")]
    pub date: Option<String>,

    /// Type
    #[arg(short = 't', long = "type", value_name = "TYPE")]
    pub salt: Option<String>,

    /// Extract date and number from name
    #[arg(short = 'x', long)]
    pub extract: bool,
}
