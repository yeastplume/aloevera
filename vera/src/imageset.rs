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

//! Internal representation for all types of artifact that hold
//! a series of images
//! This should include
//! * TileSets
//! * Sprites
//! * Individual Images
//! * Fonts (i.e. Text Tilesets)
use permutate::Permutator;

use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::png_to_frames;
use crate::{Assemblable, AssembledPrimitive};
use crate::{Error, ErrorKind};
use crate::{VeraPalette, VeraPaletteEntry};

/// Constrain values to what's ddefined in VERA spec
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum VeraPixelDepth {
	/// 1 BPP
	BPP1 = 1,
	/// 2 BPP
	BPP2 = 2,
	/// 4 BPP
	BPP4 = 4,
	/// 8 BPP
	BPP8 = 8,
}

impl fmt::Display for VeraPixelDepth {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let out = match self {
			VeraPixelDepth::BPP1 => "1",
			VeraPixelDepth::BPP2 => "2",
			VeraPixelDepth::BPP4 => "4",
			VeraPixelDepth::BPP8 => "8",
		};
		writeln!(f, "{}", out)
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
/// Store raw pixel RGB values here as read in
/// from a source image. These values can later
/// be mapped to a palette depending on pixel settings
pub struct VeraPixel {
	/// Raw red value
	pub r: u8,
	/// Raw green value
	pub g: u8,
	/// Raw blue value
	pub b: u8,
	/// Palette index, only loaded when a palette/depth
	/// is specified
	pub pal_index: Option<u8>,
	/// if this is a 1 bpp mode, just hash with 0,0,0 instead
	/// and preserve the colour for when foreground/background
	/// is needed for a tilemap
	pub is_1bpp: bool,
	/// if 1bpp, on or off
	pub is_on: bool,
}

impl Default for VeraPixel {
	fn default() -> Self {
		Self {
			r: 0,
			g: 0,
			b: 0,
			pal_index: None,
			is_1bpp: false,
			is_on: false,
		}
	}
}

impl Hash for VeraPixel {
	/// Only hash based on upper 4 bits of rgb values, as
	/// rest are lost in the palette
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self.is_1bpp {
			false => {
				(self.r >> 4).hash(state);
				(self.g >> 4).hash(state);
				(self.b >> 4).hash(state);
			}
			true => {
				self.is_on.hash(state);
			}
		}
	}
}

/// An image itself. higher-level types that include
/// images should take care to ensure widths and heights
/// are constrained to what VERA is expecting
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VeraImage {
	/// internal id for this image
	pub id: String,
	/// Image width in pixels
	width: u32,
	/// Image height in pixels
	height: u32,
	/// Data always represented internally as 8bpp
	/// regardless of final depth
	pub data: Vec<VeraPixel>,
	/// Offset into the palette, will change depending
	/// on colour depth
	pub pal_offset: u8,
	/// Intended color depth
	pub depth: VeraPixelDepth,
	/// Foreground colour on 1BPP mode
	pub foreground: u8,
	/// Background colour on 1BPP mode
	pub background: u8,
}

impl Hash for VeraImage {
	/// Don't hash id
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.width.hash(state);
		self.height.hash(state);
		for d in self.data.iter() {
			d.hash(state);
		}
	}
}

impl fmt::Display for VeraImage {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f)?;
		write!(f, "Image Details - ")?;
		writeln!(
			f,
			"ID:{}, Width: {}, Height: {}, Depth: {}",
			self.id, self.width as usize, self.height as usize, self.depth
		)?;
		writeln!(
			f,
			"Palette Offset: {}, Foreground: {:?}, Background: {:?}",
			self.pal_offset, self.foreground, self.background
		)?;
		for i in 0..self.height as usize {
			for j in 0..self.width as usize {
				let index = match self.depth {
					VeraPixelDepth::BPP1 => match self.data[i * self.width as usize + j].is_on {
						true => "1".into(),
						false => "0".into(),
					},
					_ => match self.data[i * self.width as usize + j].pal_index {
						Some(i) => format!("{}", i),
						None => "X".into(),
					},
				};
				write!(f, "{number:>width$}", number = index, width = 4)?;
			}
			writeln!(f)?;
		}
		writeln!(f, "Size in bytes: {}", self.size())?;
		writeln!(f)
	}
}

impl VeraImage {
	/// new
	pub fn new(id: &str, width: u32, height: u32) -> Self {
		Self {
			id: id.into(),
			width,
			height,
			data: vec![],
			pal_offset: 0,
			depth: VeraPixelDepth::BPP8,
			foreground: 0,
			background: 0,
		}
	}

	/// push an pixel value
	pub fn push_pixel(&mut self, r: u8, g: u8, b: u8, pal_index: Option<u8>) {
		self.data.push(VeraPixel {
			r,
			g,
			b,
			pal_index,
			is_1bpp: false,
			is_on: false,
		});
	}

	/// Size in memory in bytes
	pub fn size(&self) -> usize {
		self.data.len() * self.depth as usize / 8
	}

	/// Get the pixel value at an index
	pub fn pixel_at_index(&self, index: usize) -> Result<&VeraPixel, Error> {
		if index >= self.data.len() {
			return Err(ErrorKind::ImageIndexMissing(index).into());
		}
		Ok(&self.data[index])
	}

	/// As above, for an x / y location
	pub fn pixel_at_coord(&self, x: usize, y: usize) -> Result<&VeraPixel, Error> {
		if x >= self.width as usize || y >= self.height as usize {
			return Err(ErrorKind::InvalidImageCoords(x, y).into());
		}
		self.pixel_at_index(y * self.width as usize + x)
	}

	/// Calculate image hash
	pub fn calc_hash(&self) -> u64 {
		let mut hasher = DefaultHasher::new();
		//error!("{:?}", self);
		self.hash(&mut hasher);
		hasher.finish()
	}
}

#[derive(Clone, Copy, Debug)]
/// Image set load configuration
pub struct VeraImageSetLoadConfig {
	/// Whether to cull duplicate frames
	pub cull_duplicates: bool,
}

impl Default for VeraImageSetLoadConfig {
	fn default() -> Self {
		Self {
			cull_duplicates: true,
		}
	}
}

/// An image set itself, basically an array of images (frames)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VeraImageSet {
	/// Id of this imageset
	pub id: String,
	/// Individual image width
	pub frame_width: u32,
	/// Individual image height
	pub frame_height: u32,
	/// Target BPP
	pub depth: Option<VeraPixelDepth>,
	/// Frames per row
	frames_per_row: u32,
	/// Frames per col
	frames_per_col: u32,
	/// Image frames themselves
	pub frame_data: Vec<VeraImage>,
	/// whether frame data has been culled
	pub culled: bool,
	/// whether this imageset has been formatted
	pub formatted: bool,
}

impl VeraImageSet {
	/// Create a new image set
	pub fn new(id: &str, frame_width: u32, frame_height: u32) -> Self {
		let mut retval = VeraImageSet {
			id: id.into(),
			frame_width,
			frame_height,
			depth: None,
			frames_per_row: 0,
			frames_per_col: 0,
			frame_data: vec![],
			culled: false,
			formatted: false,
		};
		retval.reset();
		retval
	}

	fn reset(&mut self) {
		self.frames_per_row = 0;
		self.frame_data = vec![];
		self.culled = false;
		self.formatted = false;
	}

	/// Get a frame at an index
	pub fn frame_at(&self, index: usize) -> Result<&VeraImage, Error> {
		if index >= self.frame_data.len() {
			return Err(ErrorKind::FrameDataMissing(index).into());
		}
		Ok(&self.frame_data[index])
	}

	/// As above, for an x / y location
	pub fn frame_at_coord(&self, x: usize, y: usize) -> Result<&VeraImage, Error> {
		if self.culled {
			return Err(ErrorKind::ImageSetCulled.into());
		}
		if x >= self.frames_per_row as usize || y >= self.frames_per_col as usize {
			return Err(ErrorKind::InvalidFrameCoords(x, y).into());
		}
		self.frame_at(y * self.frames_per_row as usize + x)
	}

	/// Size in memory
	// TODO: Adjust for depth
	pub fn size(&self) -> usize {
		self.frame_data.iter().fold(0, |acc, t| acc + t.size())
	}

	/// Remove duplicates
	pub fn remove_duplicate_frames(&mut self) -> Result<(), Error> {
		// could be more efficient than hashing every time, but these
		// are going to be small data sets
		let mut hashes_to_indices = BTreeMap::new();
		// only keep first instance of each tile
		for (i, t) in self.frame_data.iter().enumerate() {
			let hash = t.calc_hash();
			if !hashes_to_indices.contains_key(&hash) {
				hashes_to_indices.insert(hash, i);
			} else {
				self.culled = true;
			}
		}
		debug!("Hashes to indices map: {:?}", hashes_to_indices);
		self.frame_data = self
			.frame_data
			.clone()
			.into_iter()
			.enumerate()
			.filter(|(i, _)| hashes_to_indices.values().any(|val| val == i))
			.map(|(_, t)| t)
			.collect();
		debug!("Imageset culled? - {}", self.culled);

		Ok(())
	}

	/// Load raw RGB values from a png
	/// Use indices according to the palette as it currently exists
	/// Then load indices from colour values, which will fail if any colour value
	/// in the PNG can't be found in the palette
	pub fn load_from_png(
		&mut self,
		png_data: Vec<u8>,
		config: &VeraImageSetLoadConfig,
	) -> Result<(), Error> {
		self.reset();

		let (f_r, f_c) = png_to_frames(
			&self.id,
			self.frame_width,
			self.frame_height,
			png_data,
			&mut self.frame_data,
		)?;

		self.frames_per_row = f_r;
		self.frames_per_col = f_c;

		if config.cull_duplicates {
			self.remove_duplicate_frames()?;
		}
		info!("Image parsed successfully");
		Ok(())
	}

	/// Format the stored indices with a given palette and colour depth
	/// Should fail if any frame in the set contains a range of colours
	/// that can't be found within a single 2^BPP length range in the
	/// palette. If BPP is 1, just store on or off and disregard palette
	pub fn format_indices(
		&mut self,
		palette: &VeraPalette,
		depth: VeraPixelDepth,
	) -> Result<(), Error> {
		if self.frame_data.is_empty() {
			return Err(ErrorKind::ImageSetEmpty(self.id.clone()).into());
		}
		let palette_range = 2u32.pow(depth as u32) - 1;
		info!(
			"Formatting imageset {} to palette at depth of {}",
			self.id, depth
		);
		for frame in self.frame_data.iter_mut() {
			if depth == VeraPixelDepth::BPP1 {
				for mut p in frame.data.iter_mut() {
					p.is_1bpp = true;
					if p.r == 0 && p.g == 0 && p.b == 0 {
						p.is_on = false;
					} else {
						p.is_on = true;
					}
				}
				frame.depth = depth;
				continue;
			} else {
				// remove the 1 BPP flag from all pixels
				for mut p in frame.data.iter_mut() {
					p.is_1bpp = false;
				}
			}
			let mut mappings = BTreeMap::new();
			for p in frame.data.iter_mut() {
				let (val, indices) = palette.all_indices_of_rgb(p.r, p.g, p.b);
				if indices.len() == 0 {
					return Err(ErrorKind::PaletteIndexMissing(p.r >> 4, p.g >> 4, p.b >> 4).into());
				}
				mappings.insert(val, indices);
			}
			let mut mappings: Vec<(VeraPaletteEntry, Vec<usize>)> =
				mappings.into_iter().map(|(k, v)| (k, v)).collect();

			trace!("Possible indices map: {:?}", mappings);
			let align_to_16 = match depth {
				VeraPixelDepth::BPP8 => false,
				_ => true,
			};
			let res = find_optimal_range(palette_range, &mut mappings, align_to_16)?;
			trace!("Found?: {}", res.0);
			if !res.0 {
				return Err(ErrorKind::DepthFormatError(
					frame.id.clone().into(),
					depth as u8,
					palette_range as usize,
				)
				.into());
			}
			trace!("Resolved mappings: {:?}", mappings);

			// And back to a map
			// TODO: Optimize
			let mut final_map: BTreeMap<VeraPaletteEntry, usize> = BTreeMap::new();
			for e in mappings.iter() {
				final_map.insert(e.0, e.1[0]);
			}

			// Now set the palette indices and offsets
			frame.pal_offset = res.1 as u8;
			if depth == VeraPixelDepth::BPP8 {
				frame.pal_offset = 0;
			}
			frame.depth = depth;
			for p in frame.data.iter_mut() {
				let entry = VeraPaletteEntry::new(p.r, p.g, p.b);
				let index = final_map.get(&entry);
				match index {
					Some(i) => p.pal_index = Some(*i as u8 - frame.pal_offset),
					None => {
						// shouldn't happen but belt and suspenders, etc
						return Err(
							ErrorKind::PaletteIndexMissing(p.r >> 4, p.g >> 4, p.b >> 4).into()
						);
					}
				}
			}
		}
		self.depth = Some(depth);
		self.formatted = true;
		Ok(())
	}
}

impl fmt::Display for VeraImageSet {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f)?;
		write!(f, "Image Set Details - ")?;
		writeln!(
			f,
			"ImageWidth: {}, ImageHeight: {}, Number of Images: {}, Depth: {:?}",
			self.frame_width as usize,
			self.frame_height as usize,
			self.frame_data.len(),
			self.depth
		)?;
		writeln!(f, "Size in bytes: {}", self.size())?;
		for (i, t) in self.frame_data.iter().enumerate() {
			write!(f, "Frame {} Hash: {}", i, t.calc_hash(),)?;
			writeln!(f)?;
		}
		writeln!(f)
	}
}

impl Assemblable for VeraImageSet {
	fn id(&self) -> &str {
		&self.id
	}

	fn size_in_bytes(&self) -> Result<usize, Error> {
		Ok(self.size())
	}

	fn assemble(&self) -> Result<AssembledPrimitive, Error> {
		if !self.formatted {
			return Err(ErrorKind::ImageSetNotFormatted(format!("{}", self.id)).into());
		}
		let mut retval = AssembledPrimitive::new(self.id());
		retval.add_meta(format!("{} - size is {}", self.id, self.size()));
		let depth = match self.depth {
			Some(d) => d,
			None => {
				return Err(ErrorKind::ImageSetNotFormatted(format!("{}", self.id)).into());
			}
		};
		let mut out_count = 0;
		let mut cur_out_byte = 0u8;
		let mut cur_val_count = 0;
		for frame in self.frame_data.iter() {
			for pixel in frame.data.iter() {
				let pal_index = match pixel.pal_index {
					Some(p) => p,
					None => match pixel.is_1bpp {
						true => pixel.is_on as u8,
						false => {
							return Err(
								ErrorKind::ImageSetNotFormatted(format!("{}", self.id)).into()
							);
						}
					},
				};
				if depth == VeraPixelDepth::BPP8 {
					cur_out_byte = pal_index;
					cur_val_count += 1;
				}
				if self.depth == Some(VeraPixelDepth::BPP4) {
					if pal_index > 15 {
						let msg = format!(
							"Unexpected value in {}, {} in BPP4 mode",
							self.id, pal_index
						);
						return Err(ErrorKind::UnexpectedDepthError(msg).into());
					}

					cur_out_byte <<= 4;
					cur_out_byte |= pal_index as u8;
					cur_val_count += 1;
				}
				if self.depth == Some(VeraPixelDepth::BPP2) {
					if pal_index > 3 {
						let msg = format!(
							"Unexpected value in {}, {} in BPP2 mode",
							self.id, pal_index
						);
						return Err(ErrorKind::UnexpectedDepthError(msg).into());
					}
					cur_out_byte <<= 2;
					cur_out_byte |= pal_index as u8;
					cur_val_count += 1;
				}
				if self.depth == Some(VeraPixelDepth::BPP1) {
					// Ease up palette restrictions in 1BPP mode
					cur_out_byte <<= 1;
					cur_out_byte |= pal_index as u8;
					cur_val_count += 1;
				}
				if cur_val_count == 8 / depth as usize {
					retval.add_data(&[cur_out_byte]);
					cur_out_byte = 0u8;
					cur_val_count = 0;
					out_count += 1;
				}
			}
		}
		if out_count != self.size() {
			return Err(ErrorKind::ImageSizeMismatch(out_count, self.size()).into());
		}
		Ok(retval)
	}
}

/// Attempts to find a solution for a range of possible colours in the palette
/// that fit within the depth mode
fn find_optimal_range(
	range: u32,
	range_vals: &mut Vec<(VeraPaletteEntry, Vec<usize>)>,
	align_to_16: bool,
) -> Result<(bool, usize), Error> {
	// convert to permutator lib format
	let mut lists = vec![];
	for val in range_vals.iter() {
		let mut in_vec = vec![];
		for i in val.1.iter() {
			in_vec.push(format!("{}", i));
		}
		lists.push(in_vec);
	}

	// Convert the `Vec<Vec<String>>` into a `Vec<Vec<&str>>`
	let tmp: Vec<Vec<&str>> = lists
		.iter()
		.map(|list| list.iter().map(AsRef::as_ref).collect::<Vec<&str>>())
		.collect();

	// Convert the `Vec<Vec<&str>>` into a `Vec<&[&str]>`
	let vector_of_slices: Vec<&[&str]> = tmp.iter().map(AsRef::as_ref).collect();

	// Initialize the Permutator
	let permutator = Permutator::new(&vector_of_slices);

	// iteration 2: allocates a new buffer for each permutation
	// you may opt to re-allocate or not (see iteration 1)
	for permutation in permutator {
		let max = match permutation
			.iter()
			.map(|v| usize::from_str_radix(&v, 10))
			.filter_map(Result::ok)
			.max()
		{
			Some(i) => i,
			None => {
				return Err(ErrorKind::IteratorError("Finding max index".to_string()).into());
			}
		};
		let mut min = match permutation
			.iter()
			.map(|v| usize::from_str_radix(&v, 10))
			.filter_map(Result::ok)
			.min()
		{
			Some(i) => i,
			None => {
				return Err(ErrorKind::IteratorError("Finding min index".to_string()).into());
			}
		};

		if align_to_16 {
			// Now min needs to be rounded down to the nearest multiple of 16, since pal offsets
			// are calculated as offset * 16
			min -= min % 16;
		}

		// if we're in range, we're done
		if max - min <= range as usize {
			for i in 0..permutation.len() {
				range_vals[i].1 = vec![usize::from_str_radix(permutation[i], 10)?];
				if max == min {
					break;
				}
			}
			return Ok((true, min));
		}
	}
	Ok((false, 0))
}
