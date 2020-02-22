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

//! Vera tilemap definitions

use crate::png_to_frames;
use crate::{Assemblable, AssembledPrimitive};
use crate::{Error, ErrorKind};
use crate::{VeraImageSet, VeraPalette, VeraPixelDepth};

use std::collections::BTreeMap;
use std::fmt;

/// Correspond to Vera layer tile display modes
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum VeraTileMapMode {
	/// 1 BPP 4bpp background and foreground colours
	TextBPP1_16 = 0,
	/// 1 BPP 256 Colour foreground
	TextBPP1_256 = 1,
	/// tile 2bpp
	Tile2BPP = 2,
	/// tile 4bpp
	Tile4BPP = 3,
	/// tile 8bpp
	Tile8BPP = 4,
}

impl fmt::Display for VeraTileMapMode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let out = match self {
			VeraTileMapMode::TextBPP1_16 => "TextBPP1_16",
			VeraTileMapMode::TextBPP1_256 => "TextBPP1_256",
			VeraTileMapMode::Tile2BPP => "Tile2BPP",
			VeraTileMapMode::Tile4BPP => "Tile4BPP",
			VeraTileMapMode::Tile8BPP => "Tile8BPP",
		};
		write!(f, "{}", out)
	}
}

impl VeraTileMapMode {
	///from input string
	pub fn from_input(input: &str) -> Result<VeraTileMapMode, Error> {
		match input {
			"text_16" => Ok(VeraTileMapMode::TextBPP1_16),
			"text_256" => Ok(VeraTileMapMode::TextBPP1_256),
			"tile_2bpp" => Ok(VeraTileMapMode::Tile2BPP),
			"tile_4bpp" => Ok(VeraTileMapMode::Tile4BPP),
			"tile_8bpp" => Ok(VeraTileMapMode::Tile8BPP),
			m => Err(ErrorKind::TileMapInvalidMode(m.into()).into()),
		}
	}
}

/// Types of entries corresponding to each of the above
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum VeraTileMapEntry {
	/// 8bit char index, 4 background color, 4 foreground color
	Text0(u8, u8, u8),
	/// 8bit char index, 8 foreground colour
	Text1(u8, u8),
	/// Tile modes, 8bit tile index 7:0, 4 palette offset, 1 v-flip, 1-hflip, 2 tile index 9:8
	Tile234(u16, u8, u8, u8),
}

impl fmt::Display for VeraTileMapEntry {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let out = match self {
			VeraTileMapEntry::Text0(i, _, _) => format!("{}", i),
			VeraTileMapEntry::Text1(i, _) => format!("{}", i),
			VeraTileMapEntry::Tile234(i, _, _, _) => format!("{}", i),
		};
		write!(f, "{}", out)
	}
}

impl VeraTileMapEntry {
	///get index as value
	pub fn index_as_u32(&self) -> u32 {
		match self {
			VeraTileMapEntry::Text0(i, _, _) => *i as u32,
			VeraTileMapEntry::Text1(i, _) => *i as u32,
			VeraTileMapEntry::Tile234(i, _, _, _) => *i as u32,
		}
	}
}

impl Assemblable for VeraTileMapEntry {
	fn id(&self) -> &str {
		"0"
	}

	fn assemble(&self) -> Result<AssembledPrimitive, Error> {
		let mut retval = AssembledPrimitive::new(self.id());
		let out_bytes = match self {
			VeraTileMapEntry::Text0(index, foreground, background) => {
				let mut byte_1: u8 = background << 4;
				byte_1 |= *foreground;
				(*index, byte_1)
			}
			VeraTileMapEntry::Text1(index, foreground) => (*index, *foreground),
			VeraTileMapEntry::Tile234(index, pal_offset, _, _) => {
				let byte0 = (index & 0x00ff) as u8;
				let mut byte1 = pal_offset / 16 << 4;
				let high_index = (index & 0xff00) >> 14;
				byte1 |= high_index as u8;
				(byte0, byte1)
			}
		};
		retval.add_data(&[out_bytes.0, out_bytes.1]);
		Ok(retval)
	}
}

/// Correspond to Vera layer tile display modes
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum VeraTileMapDim {
	/// 32 Tiles
	Dim32 = 0,
	/// 64 Tiles
	Dim64 = 1,
	/// 128 Tiles
	Dim128 = 2,
	/// 256 Tiles
	Dim256 = 3,
}

impl fmt::Display for VeraTileMapDim {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let out = match self {
			VeraTileMapDim::Dim32 => 32,
			VeraTileMapDim::Dim64 => 64,
			VeraTileMapDim::Dim128 => 128,
			VeraTileMapDim::Dim256 => 256,
		};
		write!(f, "{}", out)
	}
}

impl VeraTileMapDim {
	fn val_as_u32(&self) -> u32 {
		match self {
			VeraTileMapDim::Dim32 => 32,
			VeraTileMapDim::Dim64 => 64,
			VeraTileMapDim::Dim128 => 128,
			VeraTileMapDim::Dim256 => 256,
		}
	}

	/// Attempt to parse into into enum
	pub fn from_u32(val: u32) -> Result<VeraTileMapDim, Error> {
		match val {
			32 => Ok(VeraTileMapDim::Dim32),
			64 => Ok(VeraTileMapDim::Dim64),
			128 => Ok(VeraTileMapDim::Dim128),
			256 => Ok(VeraTileMapDim::Dim256),
			e => Err(ErrorKind::TileInvalidDimension(e).into()),
		}
	}
}

/// And acceptable values for tilemap dimensions
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum VeraTileDim {
	/// 8
	Dim8 = 0,
	/// 16
	Dim16 = 1,
}

impl fmt::Display for VeraTileDim {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let out = match self {
			VeraTileDim::Dim8 => "Dim8",
			VeraTileDim::Dim16 => "Dim16",
		};
		write!(f, "{}", out)
	}
}

impl VeraTileDim {
	fn val_as_u32(&self) -> u32 {
		match self {
			VeraTileDim::Dim8 => 8,
			VeraTileDim::Dim16 => 16,
		}
	}
}

impl VeraTileDim {
	/// allow permitted values, error otherwise
	pub fn from_u32(value: u32) -> Result<VeraTileDim, Error> {
		match value {
			8 => Ok(VeraTileDim::Dim8),
			16 => Ok(VeraTileDim::Dim16),
			_ => Err(ErrorKind::TileIncorrectDimension(value).into()),
		}
	}
}

/// The tilemap itself
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct VeraTileMap {
	/// id
	pub id: String,
	/// map mode
	pub mode: VeraTileMapMode,

	/// map width
	map_width: VeraTileMapDim,

	/// map height
	map_height: VeraTileMapDim,

	/// pane width of actual image created by tileset
	pane_width: Option<u32>,

	/// pane height
	pane_height: Option<u32>,

	/// Target start position on the entire map for this given 'pane'
	pane_start_x: u32,

	/// Target start position on the entire map for this given 'pane'
	pane_start_y: u32,

	/// Tile width
	tile_width: VeraTileDim,

	/// Tile height
	tile_height: VeraTileDim,

	/// Map data itself
	tiles: Vec<VeraTileMapEntry>,

	/// Also going to keep a map of tile hashes to indices/pal offset when initialized
	/// from an imageset
	imageset_entries: BTreeMap<u64, (usize, u8)>,
}

impl fmt::Display for VeraTileMap {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f)?;
		write!(f, "Tilemap Details - ")?;
		writeln!(
			f,
			"Mode: {}, Map Width: {}, Map Height: {}, Tile Width: {}, Tile Height: {}",
			self.mode, self.map_width, self.map_height, self.tile_width, self.tile_height
		)?;
		let w = match self.pane_width {
			Some(w) => w,
			None => self.map_width.val_as_u32(),
		};
		let h = match self.pane_height {
			Some(h) => h,
			None => self.map_height.val_as_u32(),
		};
		writeln!(f, "Pane width: {}, Pane Height: {}", w, h)?;
		for i in 0..h as usize {
			for j in 0..w as usize {
				let tile = self.tiles[i * w as usize + j];
				write!(f, "{:03} ", tile.index_as_u32())?;
			}
			writeln!(f)?;
		}
		writeln!(f, "Size in bytes: {}", self.size())?;
		writeln!(f)
	}
}

impl VeraTileMap {
	/// initialize values from a palette-formatted image set, returning
	/// failure if values in the imageset don't reconcile
	pub fn init_from_imageset(
		id: &str,
		mode: VeraTileMapMode,
		map_width: VeraTileMapDim,
		map_height: VeraTileMapDim,
		imageset: &VeraImageSet,
	) -> Result<VeraTileMap, Error> {
		// TODO: Check depth
		let mut res = VeraTileMap {
			id: id.to_owned(),
			mode,
			map_width,
			map_height,
			tile_width: VeraTileDim::from_u32(imageset.frame_width)?,
			tile_height: VeraTileDim::from_u32(imageset.frame_height)?,
			pane_width: None,
			pane_height: None,
			pane_start_x: 0,
			pane_start_y: 0,
			tiles: vec![],
			imageset_entries: BTreeMap::new(),
		};
		// Tile Indices init here
		for (i, f) in imageset.frame_data.iter().enumerate() {
			res.imageset_entries
				.insert(f.calc_hash(), (i, f.pal_offset));
		}
		Ok(res)
	}

	/// size in bytes
	pub fn size(&self) -> usize {
		self.tiles.len() * 2
	}

	/// Return data formatted appropriately for the current map mode
	fn entry_from_image(
		&self,
		index: u16,
		pal_offset: u8,
		foreground: u8,
		background: u8,
	) -> Result<VeraTileMapEntry, Error> {
		match self.mode {
			VeraTileMapMode::Tile2BPP | VeraTileMapMode::Tile4BPP | VeraTileMapMode::Tile8BPP => {
				Ok(VeraTileMapEntry::Tile234(index, pal_offset, 0, 0))
			}
			VeraTileMapMode::TextBPP1_16 => {
				Ok(VeraTileMapEntry::Text0(index as u8, foreground, background))
			}
			VeraTileMapMode::TextBPP1_256 => Ok(VeraTileMapEntry::Text1(index as u8, foreground)),
		}
	}

	/// Load from a PNG on which the tilemap has been painted
	/// must be same dimensions, contain tiles of same size and
	/// palette, etc, etc
	/// Palette for 1BPP modes only, to reconcile foreground colours
	/// which will be the first non-zero colour in the tile (and must be in
	/// the given palette)
	/// clear index is the index that 0 (off) will be mapped to
	pub fn load_from_png(
		&mut self,
		png_data: Vec<u8>,
		palette: Option<&VeraPalette>,
		pane_start_x: u32,
		pane_start_y: u32,
		clear_index: u8,
	) -> Result<(), Error> {
		self.tiles = vec![];
		// load as we do for an imageset
		let mut frames = vec![];
		let (frames_per_row, frames_per_col) = png_to_frames(
			&self.id,
			self.tile_width.val_as_u32(),
			self.tile_height.val_as_u32(),
			png_data,
			&mut frames,
		)?;
		if frames_per_row > self.map_width.val_as_u32()
			|| frames_per_col > self.map_height.val_as_u32()
		{
			return Err(ErrorKind::TileMapImageWrongSize(
				self.map_width.val_as_u32() as usize,
				self.map_height.val_as_u32() as usize,
				frames_per_row as usize,
				frames_per_col as usize,
			)
			.into());
		}

		self.pane_width = Some(frames_per_row);
		self.pane_height = Some(frames_per_col);

		if pane_start_x + frames_per_row > self.map_width.val_as_u32()
			|| pane_start_y + frames_per_col > self.map_width.val_as_u32()
		{
			return Err(ErrorKind::TileMapInvalidPanePos(pane_start_x, pane_start_y).into());
		}

		self.pane_start_x = pane_start_x;
		self.pane_start_y = pane_start_y;

		for (i, mut f) in frames.iter_mut().enumerate() {
			if self.mode == VeraTileMapMode::TextBPP1_16
				|| self.mode == VeraTileMapMode::TextBPP1_256
			{
				f.depth = VeraPixelDepth::BPP1;
				for mut p in f.data.iter_mut() {
					// As with imagesets, remove colour data, just use
					// on/off
					p.is_1bpp = true;
					if p.r != 0 && p.g != 0 && p.b != 0 {
						p.is_on = true;
						// store first non-zero pixel as foreground colour
						if f.foreground == 0 {
							let res = palette.unwrap().index_of_rgb(p.r, p.g, p.b);
							let index = match res {
								Some(i) => i as u8,
								None => {
									return Err(ErrorKind::PaletteIndexMissing(
										p.r >> 4,
										p.g >> 4,
										p.b >> 4,
									)
									.into());
								}
							};
							if self.mode == VeraTileMapMode::TextBPP1_16 {
								// in this mode, foreground colour must be in first palette entries
								if index > 15 {
									return Err(ErrorKind::UnexpectedDepthError(format!(
										"Palette index {} in frame {} exceeds mode depth",
										index, f.id
									))
									.into());
								}
							}
							f.foreground = index;
						}
					} else {
						f.background = clear_index;
					}
				}
			} else {
				if palette.is_some() {
					warn!("Palette not required for this map mode, ignoring");
				}
			}
			let hash = f.calc_hash();
			match self.imageset_entries.get(&hash) {
				Some((index, pal_offset)) => {
					self.tiles.push(self.entry_from_image(
						*index as u16,
						*pal_offset,
						f.foreground,
						f.background,
					)?);
				}
				None => {
					let y = i / frames_per_row as usize;
					let x = i - frames_per_row as usize * y;
					return Err(ErrorKind::TileMapNoImageSetEntry(i, x, y).into());
				}
			}
		}
		Ok(())
	}

	/// Calculate the required start index and
	pub fn calc_start_index_stride_and_skip(&self) -> (u32, u32, u32) {
		let start_index =
			self.pane_start_y * self.map_width.val_as_u32() * 2 + self.pane_start_x * 2;
		let pane_width = match self.pane_width {
			Some(w) => w,
			None => 0,
		};
		let stride = pane_width * 2;
		let skip = self.map_width.val_as_u32() * 2 - pane_width * 2;
		(start_index, stride, skip)
	}
}

impl Assemblable for VeraTileMap {
	fn id(&self) -> &str {
		&self.id
	}

	fn assemble(&self) -> Result<AssembledPrimitive, Error> {
		if self.tiles.is_empty() {
			warn!("tilemap is empty: {}", self.id);
		}
		let mut retval = AssembledPrimitive::new(self.id());
		// load instructions
		let (start_index, stride, skip) = self.calc_start_index_stride_and_skip();
		let length = self.map_width.val_as_u32() * self.map_height.val_as_u32() * 2;
		let mut conflated_meta = vec![];
		conflated_meta.push(format!("{} - size is {}", self.id, length));
		conflated_meta.push(format!(
			"{}x{} 2 byte Tilemap entries",
			self.map_width.val_as_u32(),
			self.map_height.val_as_u32()
		));
		retval.set_tilemap_conflate_info(start_index, stride, skip, length, conflated_meta);
		retval.add_meta(format!("{} size is {}", self.id, self.size()));
		retval.add_meta(format!(
			"Start write into map_data addr + ${:02X}",
			start_index
		));
		retval.add_meta(format!("read {} to write addr", stride));
		retval.add_meta(format!("skip {} write positions", skip));
		retval.add_meta(format!("repeat until {} bytes written", self.size()));

		for e in self.tiles.iter() {
			let entry_asm = e.assemble()?;
			retval.add_prim(entry_asm);
		}
		Ok(retval)
	}
}
