// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

/// Setting for the time for Asr prayer.
///
/// For Hanafi madhab, Asr is roughly an hour later than the Shafi madhab.
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(u8)]
pub enum Madhab {
    #[default]
    Shafi = 1,
    Hanafi = 2,
}

impl Madhab {
    #[must_use]
    pub const fn shadow(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::from_shafi(Madhab::Shafi, 1)]
    #[case::from_hanafi(Madhab::Hanafi, 2)]
    fn test_shadow_length(#[case] madhab: Madhab, #[case] length: u8) {
        assert_eq!(madhab.shadow(), length);
    }
}
