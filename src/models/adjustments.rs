// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use std::default::Default;

/// Time adjustment for all prayer times.
/// The value is specified in *minutes* and
/// can be either positive or negative.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, derive_builder::Builder)]
#[builder(name = "Adjustment")]
pub struct TimeAdjustment {
    #[builder(default)]
    pub fajr: i64,
    #[builder(default)]
    pub sunrise: i64,
    #[builder(default)]
    pub dhuhr: i64,
    #[builder(default)]
    pub asr: i64,
    #[builder(default)]
    pub maghrib: i64,
    #[builder(default)]
    pub isha: i64,
}

impl TimeAdjustment {
    #[must_use]
    pub const fn new(fajr: i64, sunrise: i64, dhuhr: i64, asr: i64, maghrib: i64, isha: i64) -> Self {
        Self {
            fajr,
            sunrise,
            dhuhr,
            asr,
            maghrib,
            isha,
        }
    }
}
