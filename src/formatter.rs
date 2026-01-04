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

use chrono::{Datelike, NaiveDate};

pub struct DecodedName {
    pub name: String,
    pub date: NaiveDate,
    pub number: u32,
    pub salt: Option<String>,
}

impl DecodedName {
    pub fn new(name: String, date: NaiveDate, number: u32, salt: &str) -> Self {
        let salt = if salt.is_empty() {
            None
        } else {
            Some(salt.to_string())
        };

        Self {
            name,
            date,
            number,
            salt,
        }
    }

    pub fn display(&self) {
        println!("\x1b[1;4m{}\x1b[0m", self.name);

        let prefix = if self.salt.is_some() { "No." } else { "Op." };

        let date_str = format_date_relative(self.date);

        if let Some(ref salt) = self.salt {
            println!("[{}] {} {}, dated {}", salt, prefix, self.number, date_str);
        } else {
            println!("{} {}, dated {}", prefix, self.number, date_str);
        }
    }
}

pub fn format_date_relative(date: NaiveDate) -> String {
    let today = chrono::Local::now().date_naive();
    let days_diff = (date - today).num_days();

    let date_formatted = format!("{}.{}.{}", date.day(), date.month(), date.year());

    let relative = match days_diff {
        0 => "(today)".to_string(),
        -1 => "(yesterday)".to_string(),
        1 => "(tomorrow)".to_string(),
        n if n < 0 => {
            let abs_days = n.abs();
            if abs_days == 1 {
                "(1 day ago)".to_string()
            } else {
                format!("({} days ago)", abs_days)
            }
        }
        n => {
            format!("(in {} days, yet to come!)", n)
        }
    };

    format!("{} {}", date_formatted, relative)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn mock_date_relative(date: NaiveDate, reference: NaiveDate) -> String {
        let days_diff = (date - reference).num_days();
        let date_formatted = format!("{}.{}.{}", date.day(), date.month(), date.year());

        let relative = match days_diff {
            0 => "(today)".to_string(),
            -1 => "(yesterday)".to_string(),
            1 => "(tomorrow)".to_string(),
            n if n < 0 => {
                let abs_days = n.abs();
                if abs_days == 1 {
                    "(1 day ago)".to_string()
                } else {
                    format!("({} days ago)", abs_days)
                }
            }
            n => {
                format!("(in {} days, yet to come!)", n)
            }
        };

        format!("{} {}", date_formatted, relative)
    }

    #[test]
    fn test_format_today() {
        let reference = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let result = mock_date_relative(reference, reference);
        assert!(result.contains("15.1.2026"));
        assert!(result.contains("(today)"));
    }

    #[test]
    fn test_format_yesterday() {
        let reference = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let yesterday = reference - Duration::days(1);
        let result = mock_date_relative(yesterday, reference);
        assert!(result.contains("14.1.2026"));
        assert!(result.contains("(yesterday)"));
    }

    #[test]
    fn test_format_tomorrow() {
        let reference = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let tomorrow = reference + Duration::days(1);
        let result = mock_date_relative(tomorrow, reference);
        assert!(result.contains("16.1.2026"));
        assert!(result.contains("(tomorrow)"));
        assert!(!result.contains("yet to come"));
    }

    #[test]
    fn test_format_past_one_day() {
        let reference = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let past = reference - Duration::days(1);
        let result = mock_date_relative(past, reference);
        assert!(result.contains("(yesterday)"));
    }

    #[test]
    fn test_format_past_multiple_days() {
        let reference = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let past = reference - Duration::days(5);
        let result = mock_date_relative(past, reference);
        assert!(result.contains("10.1.2026"));
        assert!(result.contains("(5 days ago)"));
    }

    #[test]
    fn test_format_future_two_days() {
        let reference = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let future = reference + Duration::days(2);
        let result = mock_date_relative(future, reference);
        assert!(result.contains("17.1.2026"));
        assert!(result.contains("(in 2 days, yet to come!)"));
    }

    #[test]
    fn test_format_future_multiple_days() {
        let reference = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let future = reference + Duration::days(3);
        let result = mock_date_relative(future, reference);
        assert!(result.contains("18.1.2026"));
        assert!(result.contains("(in 3 days, yet to come!)"));
    }

    #[test]
    fn test_date_format_no_leading_zeros() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 4).unwrap();
        let reference = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let result = mock_date_relative(date, reference);
        assert!(result.contains("4.1.2026"));
        assert!(!result.contains("04.01.2026"));
    }

    #[test]
    fn test_prefix_with_salt() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let decoded = DecodedName::new("Test name".to_string(), date, 5, "mysalt");

        assert!(decoded.salt.is_some());
        assert_eq!(decoded.salt.as_ref().unwrap(), "mysalt");
    }

    #[test]
    fn test_prefix_without_salt() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let decoded = DecodedName::new("Test name".to_string(), date, 5, "");

        assert!(decoded.salt.is_none());
    }
}
