// img2irc (C) 2022 Sadie Powell <sadie@witchery.services>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod img;
mod irc;

use crate::img::Image;
use crate::irc::ColourType;

use clap::Parser;

use std::fs::File;
use std::io::{self, Write};
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::process;

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
    #[clap(long, short = 'w', value_name = "WIDTH")]
    pub max_width: Option<NonZeroU32>,

    /// The path to the input image.
    #[clap(value_name = "IMAGE")]
    pub source: PathBuf,

    /// The path to the output text.
    #[clap(value_name = "TARGET")]
    pub target: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    // Prepare the image for conversion.
    let mut image = Image::read(&args.source).unwrap_or_else(|err| {
        eprintln!("Unable to read {}: {}.", args.source.display(), err);
        process::exit(1);
    });
    let max_width = args.max_width.unwrap_or(args.colour_type.default_width());
    image.resize(max_width, args.max_height);

    // Convert the image to IRC formatting.
    let text = image.convert(args.colour_type);

    // Write the output to the target.
    let mut fh = match &args.target {
        Some(path) => {
            let fh = File::create(&path).unwrap_or_else(|err| {
                eprintln!("Unable to create {}: {}.", path.display(), err);
                process::exit(1);
            });
            Box::new(fh) as Box<dyn Write>
        }
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };

    if let Err(err) = write!(fh, "{}", text) {
        let path = args.target.unwrap_or(PathBuf::from("{stdout}"));
        eprintln!("Unable to write to {}: {}.", path.display(), err);
        process::exit(1);
    }
}
