// Copyright 2020 Revcore Technologies Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Types and transformation operations on VERA data structures

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

#[macro_use]
extern crate log;
extern crate aloevera_util as util;

#[macro_use]
extern crate serde_derive;

use std::fmt;
use std::str::FromStr;

mod bitmap;
mod error;
mod imageset;
mod palette;
mod png_util;
mod sprite;
mod tilemap;

pub use bitmap::VeraBitmap;
pub use error::{Error, ErrorKind};
pub use imageset::{VeraImage, VeraImageSet, VeraImageSetLoadConfig, VeraPixelDepth};
pub use palette::{VeraPalette, VeraPaletteEntry, VeraPaletteLoadConfig};
pub use png_util::png_to_frames;
pub use sprite::VeraSprite;
pub use tilemap::{VeraTileMap, VeraTileMapDim, VeraTileMapMode};

/// Enum for variations ToAsm implementors can assemble to
pub enum AsmFormat {
	/// To Basic
	Basic,
	/// ca65 assembly
	Ca65,
}

impl fmt::Display for AsmFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let out = match self {
			AsmFormat::Basic => "basic",
			AsmFormat::Ca65 => "ca65",
		};
		write!(f, "{}", out)
	}
}

/// From String
impl FromStr for AsmFormat {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"ca65" => Ok(AsmFormat::Ca65),
			"basic" => Ok(AsmFormat::Basic),
			other => Err(ErrorKind::UnknownAsmFormat(other.to_owned()).into()),
		}
	}
}

/// Trait for object that outputs its data to an assembled format
pub trait Assemblable {
	/// Assemble to particular format
	fn assemble(&self, format: &AsmFormat, line_start: &mut usize) -> Result<String, Error>;
}
