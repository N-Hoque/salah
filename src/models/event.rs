// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

/// Names of all obligatory prayers, sunrise, and Qiyam.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Prayer(Prayer),
    Sunrise,
    Qiyam,
    Restricted(Reason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prayer {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}

impl std::fmt::Display for Prayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fajr => write!(f, "Fajr"),
            Self::Dhuhr => write!(f, "Dhuhr"),
            Self::Asr => write!(f, "Asr"),
            Self::Maghrib => write!(f, "Maghrib"),
            Self::Isha => write!(f, "Isha"),
        }
    }
}

impl Prayer {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Fajr => "Fajr",
            Self::Dhuhr => "Dhuhr",
            Self::Asr => "Asr",
            Self::Maghrib => "Maghrib",
            Self::Isha => "Isha",
        }
    }

    #[must_use]
    pub fn friday_name(self) -> &'static str {
        if self == Self::Dhuhr {
            "Jumu'ah"
        } else {
            self.name()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reason {
    DuringSunrise,
    DuringSunset,
    AfterMidnight,
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prayer(p) => write!(f, "{p}"),
            Self::Sunrise => write!(f, "Sunrise"),
            Self::Qiyam => write!(f, "Qiyam"),
            Self::Restricted(Reason::DuringSunset) => write!(f, "DuringSunset"),
            Self::Restricted(Reason::DuringSunrise) => write!(f, "DuringSunrise"),
            Self::Restricted(Reason::AfterMidnight) => write!(f, "AfterMidnight"),
        }
    }
}

impl Event {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Prayer(p) => p.name(),
            Self::Sunrise => "Sunrise",
            Self::Qiyam => "Qiyam",
            Self::Restricted(Reason::DuringSunrise) => "During Sunrise (Cannot perform Fajr)",
            Self::Restricted(Reason::DuringSunset) => "During Sunset (Cannot perform Asr)",
            Self::Restricted(Reason::AfterMidnight) => "After Midnight (Cannot perform Isha)",
        }
    }

    #[must_use]
    pub fn friday_name(&self) -> &'static str {
        if let Self::Prayer(p) = self {
            p.friday_name()
        } else {
            self.name()
        }
    }

    #[must_use]
    pub const fn is_daily(&self) -> bool {
        !matches!(self, Self::Sunrise | Self::Qiyam | Self::Restricted(_))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::fajr(Event::Prayer(Prayer::Fajr), "Fajr")]
    #[case::dhuhr(Event::Prayer(Prayer::Dhuhr), "Dhuhr")]
    #[case::asr(Event::Prayer(Prayer::Asr), "Asr")]
    #[case::maghrib(Event::Prayer(Prayer::Maghrib), "Maghrib")]
    #[case::isha(Event::Prayer(Prayer::Isha), "Isha")]
    #[case::qiyam(Event::Qiyam, "Qiyam")]
    fn correct_prayer_name(#[case] prayer: Event, #[case] name: &'static str) {
        assert_eq!(prayer.name(), name);
    }

    #[test]
    fn correct_prayer_name_for_friday_prayer() {
        assert_eq!("Jumu'ah", Event::Prayer(Prayer::Dhuhr).friday_name());
    }
}
