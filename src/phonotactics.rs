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

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn ends_with_vowel(s: &str) -> bool {
    s.chars().last().is_some_and(is_vowel)
}

pub fn is_suffix_compatible(stem: &str, suffix: &str) -> bool {
    let stem_lower = stem.to_lowercase();
    let suffix_lower = suffix.to_lowercase();

    let stem_ends_consonant = !ends_with_vowel(&stem_lower);

    if matches!(suffix_lower.as_str(), "yx" | "ix" | "ax") && stem_ends_consonant {
        return false;
    }

    true
}

pub fn is_phonotactically_valid(genus: &str) -> bool {
    let lower = genus.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();

    if chars.len() < 2 {
        return true;
    }

    let last_two: String = chars[chars.len().saturating_sub(2)..].iter().collect();
    let last_three: String = chars[chars.len().saturating_sub(3)..].iter().collect();

    let bad_endings = [
        "nx", "ps", "ks", "ts", "ds", "bs", "gs", "pt", "kt", "bt", "dt", "gt", "px", "kx", "tx",
        "dx", "bx", "gx", "nk", "ng", "nq", "mph", "nph", "nth", "xc", "xp", "xk", "xt", "lx",
        "rx", "mnx", "mpx", "ntx", "nkx",
    ];

    for pattern in &bad_endings {
        if last_two.ends_with(pattern) || last_three.ends_with(pattern) {
            return false;
        }
    }

    let valid_endings = [
        "us", "os", "is", "es", "as", "um", "on", "en", "er", "or", "a", "e", "o", "i", "u", "n",
        "r", "s", "m", "l", "yx", "ix", "ax", "ex", "ox", "ma",
    ];

    for ending in &valid_endings {
        if last_two.ends_with(ending) || last_three.ends_with(ending) {
            return true;
        }
    }

    let last = chars.last().unwrap();
    matches!(
        last,
        'a' | 'e' | 'i' | 'o' | 'u' | 'n' | 'r' | 's' | 'm' | 'l'
    )
}

fn has_bad_consonant_cluster(s: &str) -> bool {
    let chars: Vec<char> = s.to_lowercase().chars().collect();

    for i in 0..chars.len().saturating_sub(2) {
        let cluster: String = chars[i..i + 3].iter().collect();

        if matches!(
            cluster.as_str(),
            "nph"
                | "mph"
                | "nth"
                | "nkh"
                | "xth"
                | "pht"
                | "ckh"
                | "tzs"
                | "tsc"
                | "psc"
                | "scht"
                | "chs"
        ) {
            return true;
        }
    }

    for i in 0..chars.len().saturating_sub(3) {
        let cluster: String = chars[i..i + 4].iter().collect();

        if matches!(
            cluster.as_str(),
            "mphn" | "nthn" | "tzsch" | "tsch" | "psch"
        ) {
            return true;
        }
    }

    false
}

pub fn check_genus_quality(genus: &str) -> bool {
    if !is_phonotactically_valid(genus) {
        return false;
    }

    if has_bad_consonant_cluster(genus) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_endings() {
        assert!(is_phonotactically_valid("Hydrocephalus"));
        assert!(is_phonotactically_valid("Neuromyster"));
        assert!(is_phonotactically_valid("Chronoptera"));
        assert!(is_phonotactically_valid("Morphogen"));
    }

    #[test]
    fn test_invalid_endings() {
        assert!(!is_phonotactically_valid("Ectoalimentnx"));
        assert!(!is_phonotactically_valid("Angustirenps"));
        assert!(!is_phonotactically_valid("Heptalymphnx"));
        assert!(!is_phonotactically_valid("Gastrokt"));
    }

    #[test]
    fn test_x_suffix_rules() {
        assert!(is_phonotactically_valid("Neohelix"));
        assert!(is_phonotactically_valid("Archaeoryx"));
        assert!(!is_phonotactically_valid("Morphonx"));
        assert!(!is_phonotactically_valid("Chronokx"));
    }

    #[test]
    fn test_yx_needs_vowel() {
        assert!(is_suffix_compatible("soma", "yx"));
        assert!(is_suffix_compatible("tela", "yx"));
        assert!(!is_suffix_compatible("ren", "yx"));
        assert!(!is_suffix_compatible("aliment", "yx"));
    }

    #[test]
    fn test_normal_suffixes() {
        assert!(is_suffix_compatible("ren", "us"));
        assert!(is_suffix_compatible("aliment", "ma"));
        assert!(is_suffix_compatible("cephal", "us"));
    }
}
