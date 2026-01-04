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

pub fn pronounceability_score(name: &str) -> f32 {
    let name_lower = name.to_lowercase();
    let chars: Vec<char> = name_lower.chars().collect();

    if chars.is_empty() {
        return 0.0;
    }

    let mut penalties = 0.0;
    let mut max_penalties = 0.0;

    let (consonant_penalty, max_consonant) = check_consonant_clusters(&chars);
    penalties += consonant_penalty;
    max_penalties += max_consonant;

    let (alternation_penalty, max_alternation) = check_alternation(&chars);
    penalties += alternation_penalty;
    max_penalties += max_alternation;

    let (length_penalty, max_length) = check_length(&chars);
    penalties += length_penalty;
    max_penalties += max_length;

    let (difficult_penalty, max_difficult) = check_difficult_combinations(&name_lower);
    penalties += difficult_penalty;
    max_penalties += max_difficult;

    if max_penalties == 0.0 {
        return 1.0;
    }

    1.0 - (penalties / max_penalties)
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}

fn check_consonant_clusters(chars: &[char]) -> (f32, f32) {
    let mut max_cluster = 0;
    let mut current_cluster = 0;

    for &c in chars {
        if !is_vowel(c) && c.is_alphabetic() {
            current_cluster += 1;
            max_cluster = max_cluster.max(current_cluster);
        } else {
            current_cluster = 0;
        }
    }

    let penalty = match max_cluster {
        0..=2 => 0.0,
        3 => 0.3,
        4 => 0.6,
        _ => 1.0,
    };

    (penalty, 1.0)
}

fn check_alternation(chars: &[char]) -> (f32, f32) {
    if chars.len() < 2 {
        return (0.0, 1.0);
    }

    let mut same_type_count = 0;
    let mut max_same_type = 0;
    let mut last_was_vowel = is_vowel(chars[0]);

    for &c in &chars[1..] {
        if !c.is_alphabetic() {
            continue;
        }

        let is_vowel_now = is_vowel(c);
        if is_vowel_now == last_was_vowel {
            same_type_count += 1;
            max_same_type = max_same_type.max(same_type_count);
        } else {
            same_type_count = 0;
        }
        last_was_vowel = is_vowel_now;
    }

    let penalty = (max_same_type as f32 * 0.15).min(1.0);
    (penalty, 1.0)
}

fn check_length(chars: &[char]) -> (f32, f32) {
    let len = chars.len();

    let penalty = match len {
        0..=15 => 0.0,
        16..=18 => 0.2,
        19..=22 => 0.4,
        _ => 0.8,
    };

    (penalty, 1.0)
}

fn check_difficult_combinations(name: &str) -> (f32, f32) {
    let difficult = [
        "xth", "pht", "chth", "rrh", "ckh", "tzsch", "tsch", "psch", "chs", "ths", "scht",
    ];

    let mut count = 0;
    for pattern in &difficult {
        if name.contains(pattern) {
            count += 1;
        }
    }

    let penalty = (count as f32 * 0.3).min(1.0);
    (penalty, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy_names() {
        assert!(pronounceability_score("Homo sapiens") > 0.7);
        assert!(pronounceability_score("Canis lupus") > 0.7);
        assert!(pronounceability_score("Felis catus") > 0.7);
    }

    #[test]
    fn test_difficult_names() {
        let bad_score = pronounceability_score("Strptxthclm");
        let good_score = pronounceability_score("Homo sapiens");
        assert!(
            bad_score < good_score,
            "Bad name '{}' should score lower than good name, got {} vs {}",
            "Strptxthclm",
            bad_score,
            good_score
        );

        assert!(pronounceability_score("Aaaaaeeeeeiiii") < 0.9);
    }

    #[test]
    fn test_consonant_clusters() {
        let score = pronounceability_score("Streptoschlerox");
        assert!(score < 0.9, "Expected score < 0.9, got {}", score);
        assert!(pronounceability_score("Pterodactyl") > 0.5);
    }
}
