// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

/// Names of all obligatory prayers, sunrise, and Qiyam.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prayer {
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
    Qiyam,
    Restricted(Reason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reason {
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
            Self::Restricted(Reason::DuringSunset) => write!(f, "DuringSunset"),
            Self::Restricted(Reason::DuringSunrise) => write!(f, "DuringSunrise"),
            Self::Restricted(Reason::AfterMidnight) => write!(f, "AfterMidnight"),
        }
    }
}

impl Prayer {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Fajr => "Fajr",
            Self::Sunrise => "Sunrise",
            Self::Dhuhr => "Dhuhr",
            Self::Asr => "Asr",
            Self::Maghrib => "Maghrib",
            Self::Isha => "Isha",
            Self::Qiyam => "Qiyam",
            Self::Restricted(Reason::DuringSunrise) => "During Sunrise (Cannot perform Fajr)",
            Self::Restricted(Reason::DuringSunset) => "During Sunset (Cannot perform Asr)",
            Self::Restricted(Reason::AfterMidnight) => "After Midnight (Cannot perform Isha)",
        }
    }

    #[must_use]
    pub const fn friday_name(&self) -> &'static str {
        match self {
            Self::Dhuhr => "Jumu'ah",
            _ => self.name(),
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
    fn correct_prayer_name(#[case] prayer: Prayer, #[case] name: &'static str) {
        assert_eq!(prayer.name(), name);
    }

    #[test]
    fn correct_prayer_name_for_friday_prayer() {
        assert_eq!("Jumu'ah", Prayer::Dhuhr.friday_name());
    }
}
