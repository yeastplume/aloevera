// Copyright 2020 Revcore Technologies Ltd.
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

//! Vera Palette definition

use crate::{Assemblable, AssembledPrimitive};
use crate::{Error, ErrorKind};
use aloevera_util::gpl::parse_gpl_from_bytes;
use std::fmt;

const PALETTE_SIZE: usize = 256;

const PALETTE_DEFAULTS: [u16; PALETTE_SIZE] = [
	0x000, 0xfff, 0x800, 0xafe, 0xc4c, 0x0c5, 0x00a, 0xee7, 0xd85, 0x640, 0xf77, 0x333, 0x777,
	0xaf6, 0x08f, 0xbbb, 0x000, 0x111, 0x222, 0x333, 0x444, 0x555, 0x666, 0x777, 0x888, 0x999,
	0xaaa, 0xbbb, 0xccc, 0xddd, 0xeee, 0xfff, 0x211, 0x433, 0x644, 0x866, 0xa88, 0xc99, 0xfbb,
	0x211, 0x422, 0x633, 0x844, 0xa55, 0xc66, 0xf77, 0x200, 0x411, 0x611, 0x822, 0xa22, 0xc33,
	0xf33, 0x200, 0x400, 0x600, 0x800, 0xa00, 0xc00, 0xf00, 0x221, 0x443, 0x664, 0x886, 0xaa8,
	0xcc9, 0xfeb, 0x211, 0x432, 0x653, 0x874, 0xa95, 0xcb6, 0xfd7, 0x210, 0x431, 0x651, 0x862,
	0xa82, 0xca3, 0xfc3, 0x210, 0x430, 0x640, 0x860, 0xa80, 0xc90, 0xfb0, 0x121, 0x343, 0x564,
	0x786, 0x9a8, 0xbc9, 0xdfb, 0x121, 0x342, 0x463, 0x684, 0x8a5, 0x9c6, 0xbf7, 0x120, 0x241,
	0x461, 0x582, 0x6a2, 0x8c3, 0x9f3, 0x120, 0x240, 0x360, 0x480, 0x5a0, 0x6c0, 0x7f0, 0x121,
	0x343, 0x465, 0x686, 0x8a8, 0x9ca, 0xbfc, 0x121, 0x242, 0x364, 0x485, 0x5a6, 0x6c8, 0x7f9,
	0x020, 0x141, 0x162, 0x283, 0x2a4, 0x3c5, 0x3f6, 0x020, 0x041, 0x061, 0x082, 0x0a2, 0x0c3,
	0x0f3, 0x122, 0x344, 0x466, 0x688, 0x8aa, 0x9cc, 0xbff, 0x122, 0x244, 0x366, 0x488, 0x5aa,
	0x6cc, 0x7ff, 0x022, 0x144, 0x166, 0x288, 0x2aa, 0x3cc, 0x3ff, 0x022, 0x044, 0x066, 0x088,
	0x0aa, 0x0cc, 0x0ff, 0x112, 0x334, 0x456, 0x668, 0x88a, 0x9ac, 0xbcf, 0x112, 0x224, 0x346,
	0x458, 0x56a, 0x68c, 0x79f, 0x002, 0x114, 0x126, 0x238, 0x24a, 0x35c, 0x36f, 0x002, 0x014,
	0x016, 0x028, 0x02a, 0x03c, 0x03f, 0x112, 0x334, 0x546, 0x768, 0x98a, 0xb9c, 0xdbf, 0x112,
	0x324, 0x436, 0x648, 0x85a, 0x96c, 0xb7f, 0x102, 0x214, 0x416, 0x528, 0x62a, 0x83c, 0x93f,
	0x102, 0x204, 0x306, 0x408, 0x50a, 0x60c, 0x70f, 0x212, 0x434, 0x646, 0x868, 0xa8a, 0xc9c,
	0xfbe, 0x211, 0x423, 0x635, 0x847, 0xa59, 0xc6b, 0xf7d, 0x201, 0x413, 0x615, 0x826, 0xa28,
	0xc3a, 0xf3c, 0x201, 0x403, 0x604, 0x806, 0xa08, 0xc09, 0xf0b,
];

/// Palette Entry, all components shoud only
/// output lower 4 bits
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct VeraPaletteEntry {
	/// Red
	pub r: u8,
	/// Green
	pub g: u8,
	/// Blue
	pub b: u8,
}

impl Default for VeraPaletteEntry {
	fn default() -> Self {
		Self { r: 0, g: 0, b: 0 }
	}
}

impl VeraPaletteEntry {
	/// new
	pub fn new(r: u8, g: u8, b: u8) -> Self {
		Self {
			r: r >> 4,
			g: g >> 4,
			b: b >> 4,
		}
	}
}

/// Vera Palette
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VeraPalette {
	/// Id
	pub id: String,
	/// Palette values
	entries: Vec<VeraPaletteEntry>,
}

impl Default for VeraPalette {
	fn default() -> VeraPalette {
		let entries: Vec<VeraPaletteEntry> = PALETTE_DEFAULTS
			.iter()
			.map(|e| VeraPaletteEntry {
				b: (e & 0x000f) as u8,
				g: (e >> 4u16 & 0x000f) as u8,
				r: (e >> 8u16 & 0x000f) as u8,
			})
			.collect();
		let id = "default";
		VeraPalette {
			id: id.to_owned(),
			entries,
		}
	}
}

impl fmt::Display for VeraPalette {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f)?;
		writeln!(f, "Palette details - Length: {}", self.entries.len())?;
		let mut i = 0;
		self.entries.iter().for_each(|e| {
			let mut val: u16 = (e.r as u16) << 8;
			val |= (e.g as u16) << 4;
			val |= e.b as u16;
			let _ = writeln!(f, "Index {}: 0x{:x}", i, val);
			i += 1;
		});
		writeln!(f)
	}
}

#[derive(Clone, Copy, Debug)]
/// Palette load configuration
pub struct VeraPaletteLoadConfig {
	/// Whether to directly load pixels as they
	/// occur in the source image, used for reading
	/// a palette directly, not derived from the
	/// image data for lower BPP modes
	pub direct_load: bool,
	/// Whether to include default blank, black entries
	pub include_defaults: bool,
	/// Whether to sort palette entries on load
	pub sort: bool,
}

impl Default for VeraPaletteLoadConfig {
	fn default() -> Self {
		Self {
			direct_load: false,
			include_defaults: true,
			sort: true,
		}
	}
}

impl VeraPalette {
	/// Init an empty palette
	pub fn blank(id: &str) -> VeraPalette {
		VeraPalette {
			id: id.to_owned(),
			entries: vec![],
		}
	}

	/// Init an empty palette
	/// with black and white values
	pub fn blank_with_defaults(id: &str) -> VeraPalette {
		let mut retval = VeraPalette {
			id: id.to_owned(),
			entries: vec![],
		};
		let _ = retval.add_entry(true, 0, 0, 0);
		let _ = retval.add_entry(true, 0xff, 0xff, 0xff);
		retval
	}

	/// Size in bytes
	pub fn size(&self) -> usize {
		self.entries.len() * 2
	}

	/// Derives a palette from the given Gimp gpl file
	pub fn derive_from_gpl(
		id: &str,
		gpl_data: Vec<u8>,
		config: &VeraPaletteLoadConfig,
	) -> Result<Self, Error> {
		let gpl_palette = match parse_gpl_from_bytes(gpl_data) {
			Ok(p) => p,
			Err(s) => {
				return Err(ErrorKind::GenericError(format!("Error: {}", s)).into());
			}
		};
		debug!(
			"Palette load: Gimp palette with {} colors",
			gpl_palette.len()
		);
		let mut palette = match config.include_defaults {
			true => VeraPalette::blank_with_defaults(id),
			false => VeraPalette::blank(id),
		};
		for color in gpl_palette.iter() {
			palette.add_entry(
				config.direct_load,
				color.0 as u8,
				color.1 as u8,
				color.2 as u8,
			)?;
		}
		if config.sort {
			palette.sort();
		}
		info!("Palette creation successful");
		Ok(palette)
	}

	/// Derives a palette from the given png image
	/// this will fail if the image > 254 distinct RGB or index values
	pub fn derive_from_png(
		id: &str,
		png_data: Vec<u8>,
		config: &VeraPaletteLoadConfig,
	) -> Result<Self, Error> {
		let decoder = png::Decoder::new(&*png_data);
		let (dec_info, mut reader) = decoder.read_info()?;
		let info = reader.info();
		if info.bit_depth == png::BitDepth::Sixteen {
			return Err(
				ErrorKind::PNGInvalid(format!("PNG must be 8 bit color depth or less")).into(),
			);
		}

		let reader_step = info.color_type.samples();

		debug!("Palette load: Decoded PNG Info: {:?}", info);
		let (step, buf) = match info.palette.clone() {
			Some(p) => {
				info!("Creating new palette from image palette");
				(3, p)
			}
			None => {
				info!("Creating new palette from image data");
				let mut buf = vec![0; dec_info.buffer_size()];
				reader.next_frame(&mut buf)?;
				(reader_step, buf)
			}
		};

		let mut palette = match config.include_defaults {
			true => VeraPalette::blank_with_defaults(id),
			false => VeraPalette::blank(id),
		};

		// if creating a new palette, pass through the image data adding all palette
		// entries until we're done or we can't anymore
		for i in (0..buf.len()).step_by(step) {
			palette.add_entry(config.direct_load, buf[i], buf[i + 1], buf[i + 2])?;
		}
		if config.sort {
			palette.sort();
		}
		info!("Palette creation successful");
		Ok(palette)
	}

	/// Return the size of said palette
	pub fn len(&self) -> usize {
		self.entries.len()
	}

	/// Whether an image contains a palette colour, and at what index
	/// note that only the upper 4 bits of each color value is used
	/// for comparison, the rest are thrown out
	/// Note this will only return the first index
	pub fn index_of_rgb(&self, r: u8, g: u8, b: u8) -> Option<usize> {
		let r = r >> 4;
		let g = g >> 4;
		let b = b >> 4;
		self.entries
			.iter()
			.position(|&e| e.r == r && e.g == g && e.b == b)
	}

	/// Return all instances of a colour in the palette
	pub fn all_indices_of_rgb(&self, r: u8, g: u8, b: u8) -> (VeraPaletteEntry, Vec<usize>) {
		let r = r >> 4;
		let g = g >> 4;
		let b = b >> 4;
		let mut ret_entry = VeraPaletteEntry::default();
		let ret_vec = self
			.entries
			.iter()
			.enumerate()
			.filter(|(_, e)| e.r == r && e.g == g && e.b == b)
			.map(|(i, e)| {
				ret_entry = e.clone();
				i
			})
			.collect();
		(ret_entry, ret_vec)
	}

	/// return entry at given index
	pub fn value_at_index(&self, index: usize) -> Result<VeraPaletteEntry, Error> {
		if index as usize >= self.entries.len() {
			return Err(ErrorKind::PaletteInvalidIndex(index).into());
		}
		Ok(self.entries[index as usize])
	}

	/// Adds an entry to the palette, failing if there's no room left
	/// Only upper 4 bits are used.
	/// Returns index if the color already exists
	/// Returns the index
	pub fn add_entry(&mut self, direct_load: bool, r: u8, g: u8, b: u8) -> Result<usize, Error> {
		if self.entries.len() >= 256 {
			return Err(ErrorKind::PaletteFull.into());
		}
		if !direct_load {
			if let Some(i) = self.index_of_rgb(r, g, b) {
				return Ok(i);
			}
		}
		let entry = VeraPaletteEntry {
			r: r >> 4,
			g: g >> 4,
			b: b >> 4,
		};
		self.entries.push(entry);
		Ok(self.entries.len().saturating_sub(1))
	}

	/// Sorts the palette in ascending order, rgb
	pub fn sort(&mut self) {
		self.entries.sort()
	}
}

impl Assemblable for VeraPalette {
	fn id(&self) -> &str {
		&self.id
	}

	fn size_in_bytes(&self, _conflated: bool) -> Result<usize, Error> {
		Ok(self.size())
	}

	fn assemble(&self) -> Result<AssembledPrimitive, Error> {
		let mut retval = AssembledPrimitive::new(self.id());
		retval.add_meta(format!("{} - size is {}", self.id, self.size()));
		for e in self.entries.iter() {
			let mut off_0 = e.g << 4;
			off_0 |= e.b;
			retval.add_data(&[off_0, e.r]);
		}
		Ok(retval)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn palette_default() {
		let palette = VeraPalette::default();
		assert_eq!(palette.entries[2].r, 8);
		assert_eq!(palette.entries[2].g, 0);
		assert_eq!(palette.entries[2].b, 0);
		// spot check
		assert_eq!(palette.entries[255].r, 0xf);
		assert_eq!(palette.entries[255].g, 0);
		assert_eq!(palette.entries[255].b, 0xb);
	}

	#[test]
	fn rgb_indices() {
		let palette = VeraPalette::default();
		// a few spot checks
		assert_eq!(palette.index_of_rgb(0x8c, 0, 0), Some(2));
		assert_eq!(palette.index_of_rgb(0x8c, 0xc1, 0x30), Some(107));
		assert_eq!(palette.index_of_rgb(0xf0, 0xff, 0xfa), Some(1));
		assert_eq!(palette.index_of_rgb(0xf0, 0x0, 0xba), Some(255));
		assert_eq!(palette.index_of_rgb(0xf0, 0x0, 0xc1), None);
	}

	#[test]
	fn rgb_adding_and_sorting() -> Result<(), Error> {
		let mut palette = VeraPalette::blank_with_defaults("my_palette");
		println!("Palette: {:?}", palette);
		assert_eq!(palette.len(), 2);

		let res = palette.add_entry(false, 0xF0, 0xA0, 0xB0)?;
		assert_eq!(res, 2);
		assert_eq!(palette.len(), 3);

		// same again should return same index, not increase size
		let res = palette.add_entry(false, 0xF0, 0xA0, 0xB0)?;
		assert_eq!(res, 2);
		assert_eq!(palette.len(), 3);

		// Add another entry
		let res = palette.add_entry(false, 0xF0, 0xA0, 0xA0)?;
		assert_eq!(res, 3);
		assert_eq!(palette.len(), 4);

		// And a few more to test sort
		palette.add_entry(true, 0x00, 0x10, 0xA0)?;
		palette.add_entry(true, 0x10, 0xA0, 0x10)?;
		palette.add_entry(true, 0xC0, 0x00, 0x10)?;

		palette.sort();

		assert_eq!(palette.len(), 7);
		assert_eq!(palette.index_of_rgb(0x00, 0x00, 0x00), Some(0));
		assert_eq!(palette.index_of_rgb(0x10, 0xA0, 0x10), Some(2));
		assert_eq!(palette.index_of_rgb(0xFF, 0xFF, 0xFF), Some(6));

		Ok(())
	}

	#[test]
	fn palette_assemble() -> Result<(), Error> {
		let palette = VeraPalette::default();
		let code = palette.assemble()?;
		println!("palette: {}", palette);
		let asm = code.assemble_meta(crate::AsmFormat::Ca65, false)?;
		println!("{}", asm.to_string(None)?);
		let asm = code.assemble_data(crate::AsmFormat::Ca65, false)?;
		println!("{}", asm.to_string(None)?);
		Ok(())
	}
}
