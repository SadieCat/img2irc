// img2irc (C) 2022 Sadie Powell <sadie@witchery.services>
// SPDX-License-Identifier: AGPL-3.0-or-later

use lazy_static::lazy_static;
use oklab::{srgb_to_oklab, RGB};

use std::collections::HashMap;
use std::fmt;
use std::num::NonZeroU32;
use std::str::FromStr;

type ColourCodes = HashMap<u8, (u8, u8, u8)>;

lazy_static! {
    /// Colour codes 00-15.
    static ref BASIC: ColourCodes = HashMap::from([
        (00, (0xFF, 0xFF, 0xFF)),
        (01, (0x00, 0x00, 0x00)),
        (02, (0x00, 0x00, 0x7F)),
        (03, (0x00, 0x93, 0x00)),
        (04, (0xFF, 0x00, 0x00)),
        (05, (0x7F, 0x00, 0x00)),
        (06, (0x9C, 0x00, 0x9C)),
        (07, (0xFC, 0x7F, 0x00)),
        (08, (0xFF, 0xFF, 0x00)),
        (09, (0x00, 0xFC, 0x00)),
        (10, (0x00, 0x93, 0x93)),
        (11, (0x00, 0xFF, 0xFF)),
        (12, (0x00, 0x00, 0xFC)),
        (13, (0xFF, 0x00, 0xFF)),
        (14, (0x7F, 0x7F, 0x7F)),
        (15, (0xD2, 0xD2, 0xD2)),
    ]);

    /// Colour codes 00-98.
    static ref EXTENDED: ColourCodes = HashMap::from([
        (00, (0xFF, 0xFF, 0xFF)),
        (01, (0x00, 0x00, 0x00)),
        (02, (0x00, 0x00, 0x7F)),
        (03, (0x00, 0x93, 0x00)),
        (04, (0xFF, 0x00, 0x00)),
        (05, (0x7F, 0x00, 0x00)),
        (06, (0x9C, 0x00, 0x9C)),
        (07, (0xFC, 0x7F, 0x00)),
        (08, (0xFF, 0xFF, 0x00)),
        (09, (0x00, 0xFC, 0x00)),
        (10, (0x00, 0x93, 0x93)),
        (11, (0x00, 0xFF, 0xFF)),
        (12, (0x00, 0x00, 0xFC)),
        (13, (0xFF, 0x00, 0xFF)),
        (14, (0x7F, 0x7F, 0x7F)),
        (15, (0xD2, 0xD2, 0xD2)),
        (16, (0x47, 0x00, 0x00)),
        (17, (0x47, 0x21, 0x00)),
        (18, (0x47, 0x47, 0x00)),
        (19, (0x32, 0x47, 0x00)),
        (20, (0x00, 0x47, 0x00)),
        (21, (0x00, 0x47, 0x2C)),
        (22, (0x00, 0x47, 0x47)),
        (23, (0x00, 0x27, 0x47)),
        (24, (0x00, 0x00, 0x47)),
        (25, (0x2E, 0x00, 0x47)),
        (26, (0x47, 0x00, 0x47)),
        (27, (0x47, 0x00, 0x2A)),
        (28, (0x74, 0x00, 0x00)),
        (29, (0x74, 0x3A, 0x00)),
        (30, (0x74, 0x74, 0x00)),
        (31, (0x51, 0x74, 0x00)),
        (32, (0x00, 0x74, 0x00)),
        (33, (0x00, 0x74, 0x49)),
        (34, (0x00, 0x74, 0x74)),
        (35, (0x00, 0x40, 0x74)),
        (36, (0x00, 0x00, 0x74)),
        (37, (0x4B, 0x00, 0x74)),
        (38, (0x74, 0x00, 0x74)),
        (39, (0x74, 0x00, 0x45)),
        (40, (0xB5, 0x00, 0x00)),
        (41, (0xB5, 0x63, 0x00)),
        (42, (0xB5, 0xB5, 0x00)),
        (43, (0x7D, 0xB5, 0x00)),
        (44, (0x00, 0xB5, 0x00)),
        (45, (0x00, 0xB5, 0x71)),
        (46, (0x00, 0xB5, 0xB5)),
        (47, (0x00, 0x63, 0xB5)),
        (48, (0x00, 0x00, 0xB5)),
        (49, (0x75, 0x00, 0xB5)),
        (50, (0xB5, 0x00, 0xB5)),
        (51, (0xB5, 0x00, 0x6B)),
        (52, (0xFF, 0x00, 0x00)),
        (53, (0xFF, 0x8C, 0x00)),
        (54, (0xFF, 0xFF, 0x00)),
        (55, (0xB2, 0xFF, 0x00)),
        (56, (0x00, 0xFF, 0x00)),
        (57, (0x00, 0xFF, 0xA0)),
        (58, (0x00, 0xFF, 0xFF)),
        (59, (0x00, 0x8C, 0xFF)),
        (60, (0x00, 0x00, 0xFF)),
        (61, (0xA5, 0x00, 0xFF)),
        (62, (0xFF, 0x00, 0xFF)),
        (63, (0xFF, 0x00, 0x98)),
        (64, (0xFF, 0x59, 0x59)),
        (65, (0xFF, 0xB4, 0x59)),
        (66, (0xFF, 0xFF, 0x71)),
        (67, (0xCF, 0xFF, 0x60)),
        (68, (0x6F, 0xFF, 0x6F)),
        (69, (0x65, 0xFF, 0xC9)),
        (70, (0x6D, 0xFF, 0xFF)),
        (71, (0x59, 0xB4, 0xFF)),
        (72, (0x59, 0x59, 0xFF)),
        (73, (0xC4, 0x59, 0xFF)),
        (74, (0xFF, 0x66, 0xFF)),
        (75, (0xFF, 0x59, 0xBC)),
        (76, (0xFF, 0x9C, 0x9C)),
        (77, (0xFF, 0xD3, 0x9C)),
        (78, (0xFF, 0xFF, 0x9C)),
        (79, (0xE2, 0xFF, 0x9C)),
        (80, (0x9C, 0xFF, 0x9C)),
        (81, (0x9C, 0xFF, 0xDB)),
        (82, (0x9C, 0xFF, 0xFF)),
        (83, (0x9C, 0xD3, 0xFF)),
        (84, (0x9C, 0x9C, 0xFF)),
        (85, (0xDC, 0x9C, 0xFF)),
        (86, (0xFF, 0x9C, 0xFF)),
        (87, (0xFF, 0x94, 0xD3)),
        (88, (0x00, 0x00, 0x00)),
        (89, (0x13, 0x13, 0x13)),
        (90, (0x28, 0x28, 0x28)),
        (91, (0x36, 0x36, 0x36)),
        (92, (0x4D, 0x4D, 0x4D)),
        (93, (0x65, 0x65, 0x65)),
        (94, (0x81, 0x81, 0x81)),
        (95, (0x9F, 0x9F, 0x9F)),
        (96, (0xBC, 0xBC, 0xBC)),
        (97, (0xE2, 0xE2, 0xE2)),
        (98, (0xFF, 0xFF, 0xFF)),
    ]);
}

/// Different types of IRC colour code.
pub enum ColourType {
    /// Use colour codes 00-15.
    Basic,

    /// Use colour codes 00-98.
    Extended,

    /// Use RGB colour codes (not widely supported).
    RGB,
}

impl ColourType {
    /// Finds the nearest IRC colour code to a pixel.
    fn find_nearest(pixel: &[u8], codes: &ColourCodes) -> u8 {
        let oklab_pixel = srgb_to_oklab(RGB::new(pixel[0], pixel[1], pixel[2]));
        let mut smallest_irc = 99u8;
        let mut smallest_diff = f64::MAX;
        for (irc, hex) in codes {
            let oklab_hex = oklab::srgb_to_oklab(oklab::RGB::new(hex.0, hex.1, hex.2));
            let delta_l = (oklab_hex.l - oklab_pixel.l) as f64;
            let delta_a = (oklab_hex.a - oklab_pixel.a) as f64;
            let delta_b = (oklab_hex.b - oklab_pixel.b) as f64;
            let powers = delta_l.powi(2) + delta_a.powi(2) + delta_b.powi(2);
            let diff = (powers as f64).sqrt();
            if diff < smallest_diff {
                smallest_diff = diff;
                smallest_irc = *irc;
            }
        }

        // We have the nearest code.
        return smallest_irc;
    }

    /// Converts a pixel to an IRC colour code.
    pub fn to_irc(&self, pixel: &[u8]) -> String {
        match self {
            Self::Basic => {
                let code = Self::find_nearest(pixel, &BASIC);
                format!("\x03{:0>2},{:0>2}", code, code)
            }
            Self::Extended => {
                let code = Self::find_nearest(pixel, &EXTENDED);
                format!("\x03{:0>2},{:0>2}", code, code)
            }
            Self::RGB => format!(
                "\x04{:0>2X}{:0>2X}{:0>2X},{:0>2X}{:0>2X}{:0>2X}",
                pixel[0], pixel[1], pixel[2], pixel[0], pixel[1], pixel[2]
            ),
        }
    }

    /// Determines the default line width for a colour type.
    pub fn default_width(&self) -> NonZeroU32 {
        NonZeroU32::new(match self {
            Self::Basic => 54,
            Self::Extended => 54,
            Self::RGB => 23,
        })
        .unwrap()
    }
}

impl FromStr for ColourType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "basic" => Ok(Self::Basic),
            "extended" => Ok(Self::Extended),
            "rgb" => Ok(Self::RGB),
            _ => Err("valid values: basic, extended, rgb"),
        }
    }
}

impl fmt::Display for ColourType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let display = match *self {
            Self::Basic => "basic",
            Self::Extended => "extended",
            Self::RGB => "rgb",
        };
        write!(fmt, "{}", display)
    }
}
