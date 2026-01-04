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

use crate::config::Config;
use crate::data::{Morpheme, Morphemes, Origin};
use crate::pronounceability::pronounceability_score;
use chrono::{Datelike, NaiveDate};

const MAX_QUALITY_ATTEMPTS: u32 = 100;

fn encode_date_number(date: NaiveDate, number: u32) -> u64 {
    let year = (date.year() - 2000) as u64;
    let month = date.month() as u64;
    let day = date.day() as u64;
    let num = number as u64;

    year * 1_000_000 + month * 100_000 + day * 1_000 + num
}

fn hash_salt(salt: &str) -> u64 {
    if salt.is_empty() {
        0x123456789abcdef0
    } else {
        salt.bytes().fold(0x123456789abcdef0u64, |acc, b| {
            acc.wrapping_mul(31).wrapping_add(b as u64)
        })
    }
}

fn feistel_round(value: u64, key: u64) -> u64 {
    let mut h = value.wrapping_add(key);
    h = h.wrapping_mul(0x517cc1b727220a95);
    h ^= h >> 33;
    h = h.wrapping_mul(0x2545f4914f6cdd1d);
    h ^= h >> 29;
    h & 0xFFFFFFFF
}

fn permute(x: u64, salt_hash: u64) -> u64 {
    const ROUNDS: usize = 4;
    const MASK: u64 = 0xFFFFFFFF;

    let mut left = (x >> 32) & MASK;
    let mut right = x & MASK;

    for round in 0..ROUNDS {
        let round_key = salt_hash.wrapping_mul(round as u64 + 1);
        let f_output = feistel_round(right, round_key);
        (left, right) = (right, left ^ f_output);
    }

    (left << 32) | right
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn starts_with_vowel(s: &str) -> bool {
    s.chars().next().is_some_and(is_vowel)
}

fn ends_with_vowel(s: &str) -> bool {
    s.chars().last().is_some_and(is_vowel)
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn get_safe_suffixes(morphemes: &Morphemes) -> Vec<&'static str> {
    morphemes.genus_suffixes.iter()
        .filter(|&&s| !matches!(s, "yx" | "ix" | "ax"))
        .copied()
        .collect()
}

fn assemble_genus(prefix: &Morpheme, root: &str, suffix: &str) -> String {
    let p = prefix.text.trim_end_matches('-');
    let r = root.trim_start_matches('-').trim_end_matches('-');
    let s = suffix.trim_start_matches('-');

    let connector = match prefix.origin {
        Origin::Greek => "o",
        Origin::Latin => "i",
    };

    let stem = if ends_with_vowel(p) || starts_with_vowel(r) {
        format!("{}{}", p, r)
    } else {
        format!("{}{}{}", p, connector, r)
    };

    let result = if starts_with_vowel(s) {
        if ends_with_vowel(&stem) && stem.len() > 1 {
            format!("{}{}", &stem[..stem.len() - 1], s)
        } else {
            format!("{}{}", stem, s)
        }
    } else {
        if ends_with_vowel(&stem) {
            format!("{}{}", stem, s)
        } else {
            format!("{}{}{}", stem, connector, s)
        }
    };

    capitalize_first(&result.to_lowercase())
}

fn is_name_acceptable(genus: &str, config: &Config) -> bool {
    if genus.len() > config.max_genus_length {
        return false;
    }

    let score = pronounceability_score(genus);
    score >= config.min_pronounceability_score
}

fn generate_name_internal(
    encoded: u64,
    salt_hash: u64,
    morphemes: &Morphemes,
    _config: &Config,
) -> String {
    let permuted = permute(encoded, salt_hash);

    let genus_seed = permuted & 0xFFFFFFFF;
    let species_seed = (permuted >> 32) & 0xFFFFFFFF;

    let safe_suffixes = get_safe_suffixes(morphemes);

    let prefix_idx = (genus_seed % morphemes.prefixes.len() as u64) as usize;
    let root_idx = ((genus_seed >> 8) % morphemes.roots.len() as u64) as usize;
    let suffix_idx = ((genus_seed >> 16) % safe_suffixes.len() as u64) as usize;

    let prefix = &morphemes.prefixes[prefix_idx];
    let root = morphemes.roots[root_idx];
    let genus_suffix = safe_suffixes[suffix_idx];

    let genus = assemble_genus(prefix, root, genus_suffix);

    let category = prefix.category;

    let suitable_descriptors: Vec<&str> = morphemes
        .species_descriptors
        .iter()
        .filter(|d| d.category.is_none() || d.category == Some(category))
        .map(|d| d.text)
        .collect();

    let descriptor_idx = (species_seed % suitable_descriptors.len() as u64) as usize;
    let species = suitable_descriptors[descriptor_idx];

    format!("{} {}", genus, species)
}

pub fn generate_name(
    date: NaiveDate,
    number: u32,
    salt: &str,
    morphemes: &Morphemes,
    config: &Config,
) -> String {
    let base_encoded = encode_date_number(date, number);
    let salt_hash = hash_salt(salt);

    for quality_offset in 0..MAX_QUALITY_ATTEMPTS {
        let encoded = if quality_offset == 0 {
            base_encoded
        } else {
            base_encoded.wrapping_mul(0x9e3779b97f4a7c15)
                .wrapping_add(quality_offset as u64)
        };

        let name = generate_name_internal(encoded, salt_hash, morphemes, config);
        let genus = name.split_whitespace().next().unwrap_or("");

        if is_name_acceptable(genus, config) {
            return name;
        }
    }

    generate_name_internal(base_encoded, salt_hash, morphemes, config)
}

pub fn decode_name(
    name: &str,
    salt: &str,
    morphemes: &Morphemes,
    config: &Config,
) -> Option<(NaiveDate, u32)> {
    let now = chrono::Local::now().date_naive();

    for num in config.number_min..=config.number_max {
        if generate_name(now, num, salt, morphemes, config) == name {
            return Some((now, num));
        }
    }

    for offset in 1..=30 {
        if let Some(date) = now.checked_add_signed(chrono::Duration::days(offset)) {
            for num in config.number_min..=config.number_max {
                if generate_name(date, num, salt, morphemes, config) == name {
                    return Some((date, num));
                }
            }
        }
        if let Some(date) = now.checked_sub_signed(chrono::Duration::days(offset)) {
            for num in config.number_min..=config.number_max {
                if generate_name(date, num, salt, morphemes, config) == name {
                    return Some((date, num));
                }
            }
        }
    }

    for year in config.year_start..=config.year_end {
        for month in 1..=12 {
            let days = days_in_month(year, month);
            for day in 1..=days {
                if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                    for num in config.number_min..=config.number_max {
                        if generate_name(date, num, salt, morphemes, config) == name {
                            return Some((date, num));
                        }
                    }
                }
            }
        }
    }

    None
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Category;

    #[test]
    fn test_bijectivity() {
        let morphemes = Morphemes::new();
        let config = Config::default();

        for day in 1..=5 {
            for num in 1..=10 {
                let date = NaiveDate::from_ymd_opt(2026, 1, day).unwrap();
                let name = generate_name(date, num, "", &morphemes, &config);
                let decoded = decode_name(&name, "", &morphemes, &config);

                assert_eq!(
                    decoded,
                    Some((date, num)),
                    "Failed for date={}, num={}, name={}",
                    date,
                    num,
                    name
                );
            }
        }
    }

    #[test]
    fn test_pronounceability() {
        let morphemes = Morphemes::new();
        let config = Config::default();

        for num in 1..=20 {
            let date = NaiveDate::from_ymd_opt(2026, 1, 4).unwrap();
            let name = generate_name(date, num, "", &morphemes, &config);
            let score = pronounceability_score(&name);

            assert!(
                score >= config.min_pronounceability_score,
                "Name '{}' has score {} which is below threshold {}",
                name,
                score,
                config.min_pronounceability_score
            );
        }
    }

    #[test]
    fn test_with_salt() {
        let morphemes = Morphemes::new();
        let config = Config::default();
        let date = NaiveDate::from_ymd_opt(2026, 1, 4).unwrap();

        let name1 = generate_name(date, 1, "", &morphemes, &config);
        let name2 = generate_name(date, 1, "salt", &morphemes, &config);

        assert_ne!(name1, name2, "Names with different salts should differ");

        assert_eq!(
            decode_name(&name2, "salt", &morphemes, &config),
            Some((date, 1))
        );
        assert_ne!(
            decode_name(&name2, "", &morphemes, &config),
            Some((date, 1))
        );
    }

    #[test]
    fn test_consonant_suffix_needs_connector() {
        let prefix = Morpheme {
            text: "Ecto",
            origin: Origin::Greek,
            category: Category::Position,
        };

        let result = assemble_genus(&prefix, "aliment", "ma");
        assert_eq!(result, "Ectoalimentoma");
    }

    #[test]
    fn test_vowel_suffix_no_connector() {
        let prefix = Morpheme {
            text: "Neo",
            origin: Origin::Greek,
            category: Category::Time,
        };

        let result = assemble_genus(&prefix, "morph", "us");
        assert_eq!(result, "Neomorphus");
    }

    #[test]
    fn test_both_vowels_elision() {
        let prefix = Morpheme {
            text: "Hydro",
            origin: Origin::Greek,
            category: Category::Environment,
        };

        let result = assemble_genus(&prefix, "cephala", "us");
        assert_eq!(result, "Hydrocephalus");
    }
}
