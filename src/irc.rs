// img2irc (C) 2022 Sadie Powell <sadie@witchery.services>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt;
use std::str::FromStr;

/// Different types of IRC colour code.
pub enum ColourType {
    /// Use colour codes 00-15.
    Basic,

    /// Use colour codes 00-98.
    Extended,

    /// Use RGB colour codes (not widely supported).
    RGB,
}

impl FromStr for ColourType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "basic" => Ok(ColourType::Basic),
            "extended" => Ok(ColourType::Extended),
            "rgb" => Ok(ColourType::RGB),
            _ => Err("valid values: basic, extended, rgb"),
        }
    }
}

impl fmt::Display for ColourType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let display = match *self {
            ColourType::Basic => "basic",
            ColourType::Extended => "extended",
            ColourType::RGB => "rgb",
        };
        write!(fmt, "{}", display)
    }
}
