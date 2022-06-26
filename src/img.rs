// img2irc (C) 2022 Sadie Powell <sadie@witchery.services>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::irc::{ColourType, EscapeType};

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, Pixel};

use std::num::NonZeroU32;
use std::path::PathBuf;

pub struct Image {
    /// The underlying image.
    image: DynamicImage,
}

impl Image {
    /// Converts the image to IRC formatting.
    pub fn convert(
        &mut self,
        colour_type: &ColourType,
        escape_type: &EscapeType,
        min_alpha: u8,
    ) -> String {
        // Iterate over all pixels and convert them.
        let mut buffer = String::new();
        let mut previous_colour = String::new();
        for row in 0..self.image.height() {
            for column in 0..self.image.width() {
                let pixel = self.image.get_pixel(column, row);
                let colour = colour_type.to_irc(pixel.channels(), escape_type, min_alpha);
                if colour != previous_colour {
                    buffer.push_str(&colour);
                    previous_colour = colour;
                }
                buffer.push_str("\u{2588}\u{2588}");
            }
            previous_colour.clear();

            if cfg!(windows) {
                buffer.push_str("\r\n");
            } else {
                buffer.push('\n');
            }
        }
        buffer
    }

    /// Reads an image from a file.
    pub fn read(path: &PathBuf) -> Result<Self, String> {
        let image = ImageReader::open(path)
            .and_then(|img| img.with_guessed_format())
            .map_err(|err| err.to_string())
            .and_then(|img| img.decode().map_err(|err| err.to_string()))?;

        Ok(Self { image })
    }

    /// Resizes the image to fit within the specified bounds.
    pub fn resize(&mut self, width: NonZeroU32, height: Option<NonZeroU32>) {
        let height = height.map(|num| num.get()).unwrap_or(self.image.height());
        self.image = self.image.resize(width.get(), height, FilterType::Nearest);
    }
}
