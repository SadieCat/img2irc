// img2irc (C) 2022 Sadie Powell <sadie@witchery.services>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod irc;

use crate::irc::ColourType;

use clap::Parser;

use std::num::NonZeroU32;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version)]
pub struct Args {
    /// Whether to use basic, extended, or RGB colours.
    #[clap(default_value_t = ColourType::Basic, long, short, value_name = "TYPE")]
    pub colour_type: ColourType,

    /// The maximum height of the output text.
    #[clap(long, short = 'h', value_name = "HEIGHT")]
    pub max_height: Option<NonZeroU32>,

    /// The maximum width of the output text.
    #[clap(default_value_t = NonZeroU32::new(80).unwrap(), long, short = 'w', value_name = "WIDTH")]
    pub max_width: NonZeroU32,

    /// The path to the input image.
    #[clap(value_name = "IMAGE")]
    pub source: PathBuf,

    /// The path to the output text.
    #[clap(value_name = "TARGET")]
    pub target: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
}
