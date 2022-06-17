// img2irc (C) 2022 Sadie Powell <sadie@witchery.services>
// SPDX-License-Identifier: AGPL-3.0-or-later

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;

use std::num::NonZeroU32;
use std::path::PathBuf;

pub struct Image {
    /// The underlying image.
    image: DynamicImage,
}

impl Image {
    /// Reads an image from a file.
    pub fn read(path: &PathBuf) -> Result<Self, String> {
        let image = ImageReader::open(path)
            .and_then(|img| img.with_guessed_format())
            .map_err(|err| err.to_string())
            .and_then(|img| img.decode().map_err(|err| err.to_string()))?;

        Ok(Image { image })
    }

    /// Resizes the image to fit within the specified bounds.
    pub fn resize(&mut self, width: NonZeroU32, height: Option<NonZeroU32>) {
        let height = height.map(|num| num.get()).unwrap_or(self.image.height());
        self.image = self.image.resize(width.get(), height, FilterType::Nearest);
    }
}
