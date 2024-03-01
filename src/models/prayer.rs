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

impl Prayer {
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
            Self::Qiyam => "Qiyam",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prayer_name_for_fajr_en_transliteration() {
        assert_eq!(Prayer::Fajr.name(), "Fajr");
        assert_eq!(Prayer::Sunrise.name(), "Sunrise");

        if Utc::now().weekday() == Weekday::Fri {
            assert_eq!(Prayer::Dhuhr.name(), "Jumu'ah");
        } else {
            assert_eq!(Prayer::Dhuhr.name(), "Dhuhr");
        }

        assert_eq!(Prayer::Asr.name(), "Asr");
        assert_eq!(Prayer::Maghrib.name(), "Maghrib");
        assert_eq!(Prayer::Isha.name(), "Isha");
        assert_eq!(Prayer::Qiyam.name(), "Qiyam");
    }
}
