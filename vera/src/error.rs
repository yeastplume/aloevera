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

//! Error types for crate

use std::env;
use std::fmt::{self, Display};
use std::io;
use std::num::ParseIntError;

use failure::{Backtrace, Context, Fail};
use png::DecodingError;

/// Error definition
#[derive(Debug, Fail)]
pub struct Error {
	inner: Context<ErrorKind>,
}

/// Wallet errors, mostly wrappers around underlying crypto or I/O errors.
#[derive(Debug, Fail)]
pub enum ErrorKind {
	/// IO Error
	#[fail(display = "I/O error: {}", _0)]
	IO(String),
	/// Parse Int error
	#[fail(display = "Parse Int error: {}", _0)]
	ParseIntError(ParseIntError),
	/// Unknown ASM format string
	#[fail(display = "Unknown ASM Format: {}", _0)]
	UnknownAsmFormat(String),
	/// Invalid ASM Format
	#[fail(display = "Invalide ASM Format: {}", _0)]
	InvalidAsmFormat(String),
	/// PNG Decoding Error
	#[fail(display = "PNG Decoding error: {}", _0)]
	PNGDecoding(String),
	/// PNG Decoding Error
	#[fail(
		display = "PNG Dimensions do not match tileset width / height. ({}, {}) vs ({},{})",
		_0, _1, _2, _3
	)]
	PNGIncorrectDimensions(u32, u32, u32, u32),
	/// PNG Invalid
	#[fail(display = "Invalid PNG: ")]
	PNGInvalid(String),
	/// Palette full
	#[fail(display = "Pallette Full")]
	PaletteFull,
	/// Palette full
	#[fail(
		display = "Index for color missing from palette: (#{:x}{:x}{:x})",
		_0, _1, _2
	)]
	PaletteIndexMissing(u8, u8, u8),
	/// Attempt to load a duplicate palette entry
	#[fail(
		display = "Attempt to load duplicate palette entry: (#{:x}{:x}{:x})",
		_0, _1, _2
	)]
	PaletteDuplicateEntry(u8, u8, u8),
	/// Invalid index for palette lookup
	#[fail(display = "Invalid palette index :{}", _0)]
	PaletteInvalidIndex(usize),
	/// Image at index doesn't exist
	#[fail(display = "Image at index {} doesn't exist", _0)]
	ImageIndexMissing(usize),
	/// Invalid Image Coordinate
	#[fail(display = "Image at coords {}, {} doesn't exist", _0, _1)]
	InvalidImageCoords(usize, usize),
	/// Invalid Image Index Coordinate
	#[fail(display = "Frame at coords {}, {} doesn't exist", _0, _1)]
	InvalidFrameCoords(usize, usize),
	/// Frame at index doesn't exist
	#[fail(display = "Frame at index {} doesn't exist", _0)]
	FrameDataMissing(usize),
	/// Imageset is empty
	#[fail(display = "Imageset {} is empty", _0)]
	ImageSetEmpty(String),
	/// Attempt to access culled imageset by coordinate
	#[fail(display = "Image set has been culled of duplicates. Use index instead of x, y")]
	ImageSetCulled,
	/// Depth format error, no part of palette suits colour depth
	#[fail(
		display = "No range in palette found for frame {} suiting bit depth {} (all palette entries must be within {} indices of a multiple of 16)",
		_0, _1, _2
	)]
	DepthFormatError(String, u8, usize),
	/// Image must be formatted
	#[fail(display = "Image set {} not formatted", _0)]
	ImageSetNotFormatted(String),
	/// Unexpected depth error on output
	#[fail(display = "Unexpected depth error: {}", _0)]
	UnexpectedDepthError(String),
	/// Image size mismatch
	#[fail(display = "Image expected size mismatch - {}, {}", _0, _1)]
	ImageSizeMismatch(usize, usize),
	/// Iterator errors
	#[fail(display = "Iterator error: {}", _0)]
	IteratorError(String),
	/// Iterator errors
	#[fail(display = "Invalid tile width for tile map: {}", _0)]
	TileIncorrectDimension(u32),
	/// Invalid input dimension
	#[fail(display = "Invalid tile map width: {}", _0)]
	TileInvalidDimension(u32),
	/// Invalid input dimension
	#[fail(display = "Invalid tile map mode: {}", _0)]
	TileMapInvalidMode(String),
	/// Invalid pane start position
	#[fail(display = "Invalid tile map pane position: {}, {}", _0, _1)]
	TileMapInvalidPanePos(u32, u32),
	/// Mode not yet supported
	#[fail(display = "Tilemap Mode not yet supported")]
	TileMapModeNotSupported,
	/// No tile entry found for tilemap
	#[fail(
		display = "No imageset entry found for tilemap entry at index: {}, x,y: {},{}",
		_0, _1, _2
	)]
	TileMapNoImageSetEntry(usize, usize, usize),
	/// Given tilemap image is wrong size
	#[fail(
		display = "Tile map image should fit into {} by {} tiles, found {} by {}",
		_0, _1, _2, _3
	)]
	TileMapImageWrongSize(usize, usize, usize, usize),
	/// Sprite doesn't have an Imageset
	#[fail(display = "Sprite {} doesn't reference an imageset", _0)]
	SpriteNoImageSet(String),
	/// Bitmap doesn't have an Imageset
	#[fail(display = "Bitmap {} doesn't reference an imageset", _0)]
	BitmapNoImageSet(String),
	/// Other
	#[fail(display = "Generic error: {}", _0)]
	GenericError(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let show_bt = match env::var("RUST_BACKTRACE") {
			Ok(r) => {
				if r == "1" {
					true
				} else {
					false
				}
			}
			Err(_) => false,
		};
		let backtrace = match self.backtrace() {
			Some(b) => format!("{}", b),
			None => String::from("Unknown"),
		};
		let inner_output = format!("{}", self.inner,);
		let backtrace_output = format!("\n Backtrace: {}", backtrace);
		let mut output = inner_output.clone();
		if show_bt {
			output.push_str(&backtrace_output);
		}
		Display::fmt(&output, f)
	}
}

impl Error {
	/// get cause string
	pub fn cause_string(&self) -> String {
		match self.cause() {
			Some(k) => format!("{}", k),
			None => format!("Unknown"),
		}
	}
	/// get cause
	pub fn cause(&self) -> Option<&dyn Fail> {
		self.inner.cause()
	}
	/// get backtrace
	pub fn backtrace(&self) -> Option<&Backtrace> {
		self.inner.backtrace()
	}
}

impl From<ErrorKind> for Error {
	fn from(kind: ErrorKind) -> Error {
		Error {
			inner: Context::new(kind),
		}
	}
}

impl From<Context<ErrorKind>> for Error {
	fn from(inner: Context<ErrorKind>) -> Error {
		Error { inner: inner }
	}
}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Error {
		Error {
			inner: Context::new(ErrorKind::IO(format!("{}", error))),
		}
	}
}

impl From<DecodingError> for Error {
	fn from(error: DecodingError) -> Error {
		Error {
			inner: Context::new(ErrorKind::PNGDecoding(format!("{}", error))),
		}
	}
}

impl From<ParseIntError> for Error {
	fn from(error: ParseIntError) -> Error {
		Error {
			inner: Context::new(ErrorKind::ParseIntError(error)),
		}
	}
}
