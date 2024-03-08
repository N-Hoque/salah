// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use chrono::{Datelike, Utc, Weekday};

/// Names of all obligatory prayers,
/// sunrise, and Qiyam.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prayer {
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
    Qiyam,
    FajrTomorrow,
}

impl std::fmt::Display for Prayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fajr => write!(f, "Fajr"),
            Self::Sunrise => write!(f, "Sunrise"),
            Self::Dhuhr => write!(f, "Dhuhr"),
            Self::Asr => write!(f, "Asr"),
            Self::Maghrib => write!(f, "Maghrib"),
            Self::Isha => write!(f, "Isha"),
            Self::Qiyam => write!(f, "Qiyam"),
            Self::FajrTomorrow => write!(f, "FajrTomorrow"),
        }
    }
}

impl Prayer {
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Fajr | Self::FajrTomorrow => "Fajr",
            Self::Sunrise => "Sunrise",
            Self::Dhuhr if Utc::now().weekday() == Weekday::Fri => "Jumu'ah",
            Self::Dhuhr => "Dhuhr",
            Self::Asr => "Asr",
            Self::Maghrib => "Maghrib",
            Self::Isha => "Isha",
            Self::Qiyam => "Qiyam",
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::fajr(Prayer::Fajr, "Fajr")]
    #[case::asr(Prayer::Asr, "Asr")]
    #[case::maghrib(Prayer::Maghrib, "Maghrib")]
    #[case::isha(Prayer::Isha, "Isha")]
    #[case::qiyam(Prayer::Qiyam, "Qiyam")]
    #[case::fajr_tomorrow(Prayer::FajrTomorrow, "Fajr")]
    fn correct_prayer_name_for_non_dhuhr(#[case] prayer: Prayer, #[case] name: &'static str) {
        assert_eq!(prayer.name(), name);
    }

    #[test]
    #[ignore = "time dependent (test result changes on Friday)"]
    fn correct_prayer_name_for_dhuhr_prayer() {
        assert_eq!(
            if Utc::now().weekday() == Weekday::Fri {
                "Jumu'ah"
            } else {
                "Dhuhr"
            },
            Prayer::Dhuhr.name()
        );
    }
}
