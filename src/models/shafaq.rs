// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

/// Shafaq is the twilight in the sky. Different madhabs define the appearance of
/// twilight differently. These values are used by the `MoonsightingComittee` method
/// for the different ways to calculate Isha.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shafaq {
    /// General is a combination of Ahmer and Abyad.
    General,

    /// Ahmer means the twilight is the red glow in the sky.
    /// Used by the Shafi, Maliki, and Hanbali madhabs.
    Ahmer,

    /// Abyad means the twilight is the white glow in the sky.
    /// Used by the Hanafi madhab.
    Abyad,
}
