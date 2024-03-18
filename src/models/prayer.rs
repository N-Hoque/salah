// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use chrono::{Datelike, NaiveDate, Weekday};

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
    Forbidden(ForbiddenReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForbiddenReason {
    DuringSunrise,
    DuringSunset,
    AfterMidnight,
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
            Self::Forbidden(ForbiddenReason::DuringSunset) => write!(f, "DuringSunset"),
            Self::Forbidden(ForbiddenReason::DuringSunrise) => write!(f, "DuringSunrise"),
            Self::Forbidden(ForbiddenReason::AfterMidnight) => write!(f, "AfterMidnight"),
        }
    }
}

impl Prayer {
    #[must_use]
    pub fn name(&self, date: &NaiveDate) -> &'static str {
        match self {
            Self::Fajr => "Fajr",
            Self::Sunrise => "Sunrise",
            Self::Dhuhr if date.weekday() == Weekday::Fri => "Jumu'ah",
            Self::Dhuhr => "Dhuhr",
            Self::Asr => "Asr",
            Self::Maghrib => "Maghrib",
            Self::Isha => "Isha",
            Self::Qiyam => "Qiyam",
            Self::Forbidden(ForbiddenReason::DuringSunrise) => "Forbidden (During Sunrise)",
            Self::Forbidden(ForbiddenReason::DuringSunset) => "Forbidden (During Sunset)",
            Self::Forbidden(ForbiddenReason::AfterMidnight) => "Forbidden (After Midnight)",
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::fajr(Prayer::Fajr, "Fajr")]
    #[case::dhuhr(Prayer::Dhuhr, "Dhuhr")]
    #[case::asr(Prayer::Asr, "Asr")]
    #[case::maghrib(Prayer::Maghrib, "Maghrib")]
    #[case::isha(Prayer::Isha, "Isha")]
    #[case::qiyam(Prayer::Qiyam, "Qiyam")]
    fn correct_prayer_name_for_non_dhuhr(#[case] prayer: Prayer, #[case] name: &'static str) {
        assert_eq!(prayer.name(&NaiveDate::from_ymd_opt(2024, 3, 14).unwrap()), name);
    }

    #[test]
    fn correct_prayer_name_for_friday_prayer() {
        assert_eq!(
            "Jumu'ah",
            Prayer::Dhuhr.name(&NaiveDate::from_ymd_opt(2024, 3, 15).unwrap())
        );
    }
}
