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

//! Wrapper for an Imageset Representing a Sprite
use crate::{AsmFormat, Assemblable};
use crate::{Error, ErrorKind};
use crate::{VeraImageSet, VeraPixelDepth};
use std::fmt;

/// Acceptable values for sprite dimensions
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum VeraSpriteDim {
	/// 8
	Dim8 = 0,
	/// 16
	Dim16 = 1,
	/// 32
	Dim32 = 2,
	/// 64
	Dim64 = 3,
}

impl fmt::Display for VeraSpriteDim {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let out = match self {
			VeraSpriteDim::Dim8 => "Dim8",
			VeraSpriteDim::Dim16 => "Dim16",
			VeraSpriteDim::Dim32 => "Dim32",
			VeraSpriteDim::Dim64 => "Dim64",
		};
		write!(f, "{}", out)
	}
}

impl VeraSpriteDim {
	fn _val_as_u32(&self) -> u32 {
		match self {
			VeraSpriteDim::Dim8 => 8,
			VeraSpriteDim::Dim16 => 16,
			VeraSpriteDim::Dim32 => 32,
			VeraSpriteDim::Dim64 => 64,
		}
	}
}

impl VeraSpriteDim {
	/// allow permitted values, error otherwise
	pub fn from_u32(value: u32) -> Result<VeraSpriteDim, Error> {
		match value {
			8 => Ok(VeraSpriteDim::Dim8),
			16 => Ok(VeraSpriteDim::Dim16),
			32 => Ok(VeraSpriteDim::Dim32),
			64 => Ok(VeraSpriteDim::Dim64),
			_ => Err(ErrorKind::TileIncorrectDimension(value).into()),
		}
	}
}

/// The sprite itself
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VeraSprite<'a> {
	/// id
	pub id: String,
	/// colour depth
	pub depth: VeraPixelDepth,
	/// width
	pub frame_width: VeraSpriteDim,
	/// height
	pub frame_height: VeraSpriteDim,
	/// We won't serialize this, will just need to ensure
	/// `init_from_imageset` populates this on each run
	#[serde(skip)]
	pub imageset: Option<&'a VeraImageSet>,
	/// Imageset ID, to hold on to the reference when
	/// serializing
	pub imageset_id: String,
}

impl<'a> VeraSprite<'a> {
	/// initialize values from a palette-formatted image set
	/// All we're really doing here is verifying that the
	/// imageset conforms to allowed sprite dimensions and
	/// depths
	pub fn init_from_imageset(id: &str, imageset: &'a VeraImageSet) -> Result<Self, Error> {
		if !imageset.formatted {
			return Err(ErrorKind::ImageSetNotFormatted(imageset.id.to_owned()).into());
		}
		let depth = match imageset.depth {
			Some(d) => d,
			None => {
				return Err(ErrorKind::ImageSetNotFormatted(imageset.id.to_owned()).into());
			}
		};

		if depth != VeraPixelDepth::BPP4 && depth != VeraPixelDepth::BPP8 {
			return Err(ErrorKind::UnexpectedDepthError(imageset.id.to_owned()).into());
		}
		Ok(VeraSprite {
			id: id.into(),
			imageset_id: imageset.id.clone(),
			depth,
			frame_width: VeraSpriteDim::from_u32(imageset.frame_width)?,
			frame_height: VeraSpriteDim::from_u32(imageset.frame_height)?,
			imageset: Some(imageset),
		})
	}
}

impl<'a> Assemblable for VeraSprite<'a> {
	fn assemble(&self, out_format: &AsmFormat, line_start: &mut usize) -> Result<String, Error> {
		if self.imageset.is_none() {
			return Err(ErrorKind::SpriteNoImageSet(format!("{}", self.id)).into());
		}
		let imageset = self.imageset.as_ref().unwrap();
		if imageset.frame_data.is_empty() {
			return Err(ErrorKind::ImageSetEmpty(self.id.clone()).into());
		}
		let mut retval = match out_format {
			AsmFormat::Ca65 => format!("\n;{} - Total size is {}", self.id, imageset.size()),
			AsmFormat::Basic => format!("\n{} REM SIZE IS {}", line_start, imageset.size()),
		};
		*line_start += 1;
		let frame_size = imageset.frame_data[0].size();
		retval += &match out_format {
			AsmFormat::Ca65 => format!("\n;{} - Frame size is ${:X}", self.id, frame_size),
			AsmFormat::Basic => format!(
				"\n{} REM FRAME SIZE IS ${:X}",
				line_start,
				imageset.frame_data[0].size()
			),
		};
		*line_start += 1;
		for (i, f) in imageset.frame_data.iter().enumerate() {
			retval += &match out_format {
				AsmFormat::Ca65 => format!("\n;Frame {} starts at addr + ${:X}", i, frame_size * i),
				AsmFormat::Basic => format!(
					"\n{} REM FRAME {} STARTS AT ADDR + ${:X}",
					line_start,
					i,
					frame_size * i
				),
			};
			*line_start += 1;
			retval += &match out_format {
				AsmFormat::Ca65 => format!("\n;Frame {} pal offset - {}", i, f.pal_offset),
				AsmFormat::Basic => format!(
					"\n{} REM FRAME {} PAL OFFSET - {}",
					line_start, i, f.pal_offset,
				),
			};
			*line_start += 1;
		}
		retval += "\n";
		retval += &imageset.assemble(out_format, line_start)?;
		Ok(retval)
	}
}
