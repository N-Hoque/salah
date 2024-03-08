// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

/// Setting for the time for Asr prayer.
///
/// For Hanafi madhab, Asr is roughly an hour later than the Shafi madhab.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Madhab {
    Shafi = 1,
    Hanafi = 2,
}

impl Madhab {
    #[must_use]
    pub const fn shadow(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::from_shafi(Madhab::Shafi, 1)]
    #[case::from_hanafi(Madhab::Hanafi, 2)]
    fn test_shadow_length(#[case] madhab: Madhab, #[case] length: i32) {
        assert_eq!(madhab.shadow(), length);
    }
}
