// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use chrono::{Datelike, Utc, Weekday};

/// Prayers and particular times of day
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
    Midnight,
    Qiyam,
    FajrTomorrow,
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fajr => write!(f, "Fajr"),
            Self::Sunrise => write!(f, "Sunrise"),
            Self::Dhuhr => write!(f, "Dhuhr"),
            Self::Asr => write!(f, "Asr"),
            Self::Maghrib => write!(f, "Maghrib"),
            Self::Isha => write!(f, "Isha"),
            Self::Midnight => write!(f, "Midnight"),
            Self::Qiyam => write!(f, "Qiyam"),
            Self::FajrTomorrow => write!(f, "FajrTomorrow"),
        }
    }
}

impl Event {
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Fajr | Self::FajrTomorrow => "Fajr",
            Self::Sunrise => "Sunrise",
            Self::Dhuhr => {
                if Utc::now().weekday() == Weekday::Fri {
                    "Jumu'ah"
                } else {
                    "Dhuhr"
                }
            }
            Self::Asr => "Asr",
            Self::Maghrib => "Maghrib",
            Self::Isha => "Isha",
            Self::Midnight => "Midnight",
            Self::Qiyam => "Qiyam",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prayer_name_for_fajr_en_transliteration() {
        assert_eq!(Event::Fajr.name(), "Fajr");
        assert_eq!(Event::Sunrise.name(), "Sunrise");

        if Utc::now().weekday() == Weekday::Fri {
            assert_eq!(Event::Dhuhr.name(), "Jumu'ah");
        } else {
            assert_eq!(Event::Dhuhr.name(), "Dhuhr");
        }

        assert_eq!(Event::Asr.name(), "Asr");
        assert_eq!(Event::Maghrib.name(), "Maghrib");
        assert_eq!(Event::Isha.name(), "Isha");
        assert_eq!(Event::Qiyam.name(), "Qiyam");
    }
}
