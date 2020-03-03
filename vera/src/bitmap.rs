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
use crate::{Assemblable, AssembledPrimitive};
use crate::{Error, ErrorKind};
use crate::{VeraImageSet, VeraPixelDepth};
use std::fmt;

/// Acceptable values for bitmap dimensions (mostly width)
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum VeraBitmapDim {
	/// 320
	Dim320 = 0,
	/// 640
	Dim640 = 1,
}

impl fmt::Display for VeraBitmapDim {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let out = match self {
			VeraBitmapDim::Dim320 => "Dim320",
			VeraBitmapDim::Dim640 => "Dim640",
		};
		write!(f, "{}", out)
	}
}

impl VeraBitmapDim {
	fn val_as_u32(&self) -> u32 {
		match self {
			VeraBitmapDim::Dim320 => 320,
			VeraBitmapDim::Dim640 => 640,
		}
	}
}

impl VeraBitmapDim {
	/// allow permitted values, error otherwise
	pub fn from_u32(value: u32) -> Result<VeraBitmapDim, Error> {
		match value {
			320 => Ok(VeraBitmapDim::Dim320),
			640 => Ok(VeraBitmapDim::Dim640),
			_ => Err(ErrorKind::TileIncorrectDimension(value).into()),
		}
	}
}

/// The sprite itself
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VeraBitmap<'a> {
	/// id
	pub id: String,
	/// colour depth
	pub depth: VeraPixelDepth,
	/// width
	pub width: VeraBitmapDim,
	/// We won't serialize this, will just need to ensure
	/// `init_from_imageset` populates this on each run
	#[serde(skip)]
	pub imageset: Option<&'a VeraImageSet>,
	/// Imageset ID, to hold on to the reference when
	/// serializing
	pub imageset_id: String,
}

impl<'a> VeraBitmap<'a> {
	/// initialize values from a palette-formatted image set
	/// All we're really doing here is verifying that the
	/// imageset conforms to allowed dimensions and
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

		if depth == VeraPixelDepth::BPP1 {
			return Err(ErrorKind::UnexpectedDepthError(imageset.id.to_owned()).into());
		}
		Ok(VeraBitmap {
			id: id.into(),
			imageset_id: imageset.id.clone(),
			depth,
			width: VeraBitmapDim::from_u32(imageset.frame_width)?,
			imageset: Some(imageset),
		})
	}
}

impl<'a> Assemblable for VeraBitmap<'a> {
	fn id(&self) -> &str {
		&self.id
	}

	fn size_in_bytes(&self, _conflated: bool) -> Result<usize, Error> {
		match self.imageset {
			Some(i) => Ok(i.size()),
			None => Err(ErrorKind::BitmapNoImageSet(format!("{}", self.id)).into()),
		}
	}

	fn assemble(&self) -> Result<AssembledPrimitive, Error> {
		let mut retval = AssembledPrimitive::new(self.id());
		if self.imageset.is_none() {
			return Err(ErrorKind::BitmapNoImageSet(format!("{}", self.id)).into());
		}
		let imageset = self.imageset.as_ref().unwrap();
		if imageset.frame_data.is_empty() {
			return Err(ErrorKind::ImageSetEmpty(self.id.clone()).into());
		}
		retval.add_meta(format!(
			"{} - Width is {}",
			self.id,
			self.width.val_as_u32()
		));
		let pal_offset = imageset.frame_data[0].pal_offset;
		retval.add_meta(format!("Palette offset is {}", pal_offset));
		let imageset_asm = imageset.assemble()?;
		retval.add_prim(imageset_asm);
		Ok(retval)
	}
}
