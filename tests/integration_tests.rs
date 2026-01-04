use chrono::NaiveDate;
use holotype::config::Config;
use holotype::data::Morphemes;
use holotype::generator::{decode_name, generate_name};
use holotype::pronounceability::pronounceability_score;

#[test]
fn test_full_cycle() {
    let morphemes = Morphemes::new();
    let config = Config::default();

    let date = NaiveDate::from_ymd_opt(2026, 1, 4).unwrap();
    let number = 42;
    let salt = "test_salt";

    let name = generate_name(date, number, salt, &morphemes, &config);
    let decoded = decode_name(&name, salt, &morphemes, &config);

    assert_eq!(decoded, Some((date, number)));
}

#[test]
fn test_bijectivity_comprehensive() {
    let morphemes = Morphemes::new();
    let config = Config::default();

    let test_dates = [(2000, 1, 1), (2025, 6, 15), (2050, 12, 31), (2099, 7, 4)];

    for (year, month, day) in test_dates {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        for num in [1, 10, 50, 99] {
            let name = generate_name(date, num, "", &morphemes, &config);
            let decoded = decode_name(&name, "", &morphemes, &config);

            assert_eq!(
                decoded,
                Some((date, num)),
                "Failed for date={}-{:02}-{:02}, num={}, name={}",
                year,
                month,
                day,
                num,
                name
            );
        }
    }
}

#[test]
fn test_no_collisions() {
    let morphemes = Morphemes::new();
    let config = Config::default();

    use std::collections::HashSet;
    let mut seen_names = HashSet::new();

    for year in [2020, 2030, 2040, 2050] {
        for month in 1..=12 {
            for day in [1, 15] {
                if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                    for num in 1..=10 {
                        let name = generate_name(date, num, "", &morphemes, &config);
                        assert!(
                            seen_names.insert(name.clone()),
                            "Collision found: {} generated for multiple (date, number) pairs",
                            name
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn test_salt_independence() {
    let morphemes = Morphemes::new();
    let config = Config::default();
    let date = NaiveDate::from_ymd_opt(2026, 1, 4).unwrap();

    let name1 = generate_name(date, 1, "", &morphemes, &config);
    let name2 = generate_name(date, 1, "salt1", &morphemes, &config);
    let name3 = generate_name(date, 1, "salt2", &morphemes, &config);

    assert_ne!(name1, name2);
    assert_ne!(name2, name3);
    assert_ne!(name1, name3);

    assert_eq!(
        decode_name(&name1, "", &morphemes, &config),
        Some((date, 1))
    );
    assert_eq!(
        decode_name(&name2, "salt1", &morphemes, &config),
        Some((date, 1))
    );
    assert_eq!(
        decode_name(&name3, "salt2", &morphemes, &config),
        Some((date, 1))
    );

    assert_ne!(
        decode_name(&name1, "salt1", &morphemes, &config),
        Some((date, 1))
    );
}

#[test]
fn test_pronounceability_threshold() {
    let morphemes = Morphemes::new();
    let config = Config::default();

    for year in [2020, 2050] {
        for month in 1..=12 {
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, 1) {
                for num in 1..=20 {
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
        }
    }
}

#[test]
fn test_genus_length_limit() {
    let morphemes = Morphemes::new();
    let config = Config::default();

    for num in 1..=50 {
        let date = NaiveDate::from_ymd_opt(2026, 1, 4).unwrap();
        let name = generate_name(date, num, "", &morphemes, &config);
        let genus = name.split_whitespace().next().unwrap();

        assert!(
            genus.len() <= config.max_genus_length,
            "Genus '{}' has length {} which exceeds limit {}",
            genus,
            genus.len(),
            config.max_genus_length
        );
    }
}

#[test]
fn test_config_validation() {
    let mut config = Config::default();
    assert!(config.validate().is_ok());

    config.year_start = 2100;
    config.year_end = 2000;
    assert!(config.validate().is_err());

    config = Config::default();
    config.min_pronounceability_score = 1.5;
    assert!(config.validate().is_err());
}
