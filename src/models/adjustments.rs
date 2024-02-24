// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use std::default::Default;

/// Time adjustment for all prayer times.
/// The value is specified in *minutes* and
/// can be either positive or negative.
#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct TimeAdjustment {
    pub fajr: i64,
    pub sunrise: i64,
    pub dhuhr: i64,
    pub asr: i64,
    pub maghrib: i64,
    pub isha: i64,
}

impl TimeAdjustment {
    #[must_use]
    pub fn new(fajr: i64, sunrise: i64, dhuhr: i64, asr: i64, maghrib: i64, isha: i64) -> Self {
        TimeAdjustment {
            fajr,
            sunrise,
            dhuhr,
            asr,
            maghrib,
            isha,
        }
    }
}

/// Builder struct for the [`TimeAdjustment`](struct.TimeAdjustment.html).
/// It is recommended to use this for all needed adjustments.
pub struct Adjustment {
    fajr: i64,
    sunrise: i64,
    dhuhr: i64,
    asr: i64,
    maghrib: i64,
    isha: i64,
}

impl Default for Adjustment {
    fn default() -> Self {
        Self::new()
    }
}

impl Adjustment {
    #[must_use]
    pub fn new() -> Adjustment {
        Adjustment {
            fajr: 0,
            sunrise: 0,
            dhuhr: 0,
            asr: 0,
            maghrib: 0,
            isha: 0,
        }
    }

    pub fn fajr(&mut self, fajr: i64) -> &mut Adjustment {
        self.fajr = fajr;
        self
    }

    pub fn sunrise(&mut self, sunrise: i64) -> &mut Adjustment {
        self.sunrise = sunrise;
        self
    }

    pub fn dhuhr(&mut self, dhuhr: i64) -> &mut Adjustment {
        self.dhuhr = dhuhr;
        self
    }

    pub fn asr(&mut self, asr: i64) -> &mut Adjustment {
        self.asr = asr;
        self
    }

    pub fn maghrib(&mut self, maghrib: i64) -> &mut Adjustment {
        self.maghrib = maghrib;
        self
    }

    pub fn isha(&mut self, isha: i64) -> &mut Adjustment {
        self.isha = isha;
        self
    }

    #[must_use]
    pub fn done(&self) -> TimeAdjustment {
        TimeAdjustment {
            fajr: self.fajr,
            sunrise: self.sunrise,
            dhuhr: self.dhuhr,
            asr: self.asr,
            maghrib: self.maghrib,
            isha: self.isha,
        }
    }
}
