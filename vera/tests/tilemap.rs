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

use aloevera_util::init_test_logger;
use aloevera_vera::Error;

use aloevera_vera::{AsmFormat, Assemblable};
use aloevera_vera::{
	VeraImageSet, VeraImageSetLoadConfig, VeraPalette, VeraPaletteLoadConfig, VeraPixelDepth,
	VeraTileMap, VeraTileMapDim, VeraTileMapEntry, VeraTileMapMode,
};

#[test]
fn tilemap_32_x_32_x_16_8bpp() -> Result<(), Error> {
	init_test_logger();
	let setdata = include_bytes!("data/tilemap/tileset_4-bpp.png");
	// Load Palette straigt from tilemap
	let pal_config = VeraPaletteLoadConfig {
		direct_load: true,
		include_defaults: false,
		sort: false,
		..VeraPaletteLoadConfig::default()
	};
	let palette = VeraPalette::derive_from_png("pal", setdata.to_vec(), &pal_config)?;
	println!("{}", palette);

	// create imageset from tilemap
	let mut set = VeraImageSet::new("tileset_1", 16, 16);
	let config = VeraImageSetLoadConfig::default();
	set.load_from_png(setdata.to_vec(), &config)?;
	set.format_indices(&palette, VeraPixelDepth::BPP4)?;
	println!("{}", set);

	let frame = set.frame_at(2)?;
	println!("{}", frame);

	// Init tilemap
	let mut tilemap = VeraTileMap::init_from_imageset(
		"my tilemap",
		VeraTileMapMode::Tile4BPP,
		VeraTileMapDim::Dim32,
		VeraTileMapDim::Dim32,
		&set,
	)?;

	// try an incorrect tilemap first (contains tile not found in set)
	let mapdata = include_bytes!("data/tilemap/tilemap_32x32x16_incorrect_tile.png");
	assert!(tilemap
		.load_from_png(mapdata.to_vec(), None, 0, 0, 0)
		.is_err());

	// And a correct one
	let mapdata = include_bytes!("data/tilemap/tilemap_32x32x16.png");
	tilemap.load_from_png(mapdata.to_vec(), None, 0, 0, 0)?;

	println!("{}", tilemap);
	let code = tilemap.assemble()?;
	println!("palette: {}", palette);
	let asm = code.assemble_meta(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);
	let asm = code.assemble_data(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);

	Ok(())
}

#[test]
fn tilemap_text_8_x_8() -> Result<(), Error> {
	// 1 bit per pixel, used for text modes
	// palette is less important here
	init_test_logger();
	// since we're targeting 4bpp, import indexed straight as is
	let test_png = include_bytes!("data/imageset/indexed-8-x-8-1bpp.png");
	let pal_config = VeraPaletteLoadConfig {
		direct_load: true,
		include_defaults: false,
		sort: false,
		..VeraPaletteLoadConfig::default()
	};
	let palette = VeraPalette::derive_from_png("pal", test_png.to_vec(), &pal_config)?;

	println!("{}", palette);
	let mut set = VeraImageSet::new("text_set_1", 8, 8);
	let config = VeraImageSetLoadConfig::default();
	set.load_from_png(test_png.to_vec(), &config)?;
	set.format_indices(&palette, VeraPixelDepth::BPP1)?;
	println!("{}", set);

	let frame = set.frame_at(1)?;
	println!("{}", frame);

	// Init tilemap for 1BPP mode
	let mut tilemap = VeraTileMap::init_from_imageset(
		"my tilemap",
		VeraTileMapMode::TextBPP1_16,
		VeraTileMapDim::Dim128,
		VeraTileMapDim::Dim64,
		&set,
	)?;

	let mapdata = include_bytes!("data/tilemap/tilemap-banner-1bpp.png");
	tilemap.load_from_png(mapdata.to_vec(), Some(&palette), 1, 1, 0)?;
	println!("{}", tilemap);

	let code = tilemap.assemble()?;
	let asm = code.assemble_meta(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);
	let asm = code.assemble_data(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);

	// Init tilemap again for 1BPP 256 mode
	let mut tilemap = VeraTileMap::init_from_imageset(
		"my tilemap",
		VeraTileMapMode::TextBPP1_256,
		VeraTileMapDim::Dim128,
		VeraTileMapDim::Dim64,
		&set,
	)?;

	let mapdata = include_bytes!("data/tilemap/tilemap-banner-1bpp.png");
	tilemap.load_from_png(mapdata.to_vec(), Some(&palette), 1, 1, 0)?;
	println!("{}", tilemap);

	// and output in format 1
	let code = tilemap.assemble()?;
	let asm = code.assemble_meta(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);
	let asm = code.assemble_data(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);

	// Conflated
	let _asm = code.assemble_data(crate::AsmFormat::Ca65, true)?;
	//println!("{}", asm.to_string(None)?);

	Ok(())
}

#[test]
fn tilemap_128_x_32_x_16_4bpp() -> Result<(), Error> {
	init_test_logger();
	let setdata = include_bytes!("data/tilemap/tile_wall-imageset-4bpp.png");
	// Load Palette straigt from tilemap
	let pal_config = VeraPaletteLoadConfig {
		direct_load: true,
		include_defaults: false,
		sort: false,
		..VeraPaletteLoadConfig::default()
	};
	let palette = VeraPalette::derive_from_png("pal", setdata.to_vec(), &pal_config)?;
	println!("{}", palette);

	// create imageset from tilemap
	let mut set = VeraImageSet::new("tileset_1", 16, 16);
	let config = VeraImageSetLoadConfig::default();
	set.load_from_png(setdata.to_vec(), &config)?;
	set.format_indices(&palette, VeraPixelDepth::BPP8)?;
	println!("{}", set);

	let frame = set.frame_at(3)?;
	println!("{}", frame);

	// Init tilemap
	let mut tilemap = VeraTileMap::init_from_imageset(
		"my tilemap",
		VeraTileMapMode::Tile4BPP,
		VeraTileMapDim::Dim64,
		VeraTileMapDim::Dim32,
		&set,
	)?;

	// And a correct one
	let mapdata = include_bytes!("data/tilemap/tile_wall-map.png");
	tilemap.load_from_png(mapdata.to_vec(), None, 0, 10, 0)?;
	// Check rotated tiles
	// h_flipped
	println!("Rotated tile at index 1: {}", tilemap.get_tiles()[1]);
	assert_eq!(
		tilemap.get_tiles()[1],
		VeraTileMapEntry::Tile234(14, 0, 0, 1)
	);

	// v_flipped
	println!("Rotated tile at index 2: {}", tilemap.get_tiles()[2]);
	assert_eq!(
		tilemap.get_tiles()[2],
		VeraTileMapEntry::Tile234(14, 0, 1, 0)
	);

	// both
	println!("Rotated tile at index 3: {}", tilemap.get_tiles()[3]);
	assert_eq!(
		tilemap.get_tiles()[3],
		VeraTileMapEntry::Tile234(14, 0, 1, 1)
	);

	println!("{}", tilemap);
	let code = tilemap.assemble()?;
	let asm = code.assemble_meta(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);
	let asm = code.assemble_data(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);

	Ok(())
}

#[test]
fn tilemap_64_x_32_x_8_4bpp() -> Result<(), Error> {
	init_test_logger();
	let paldata = include_bytes!("data/tilemap/palette-large.png");
	// Load Palette straigt from tilemap
	let pal_config = VeraPaletteLoadConfig {
		direct_load: true,
		include_defaults: false,
		sort: false,
		..VeraPaletteLoadConfig::default()
	};
	let palette = VeraPalette::derive_from_png("palette_error", paldata.to_vec(), &pal_config)?;
	println!("{}", palette);

	// create imageset from tilemap
	let setdata = include_bytes!("data/tilemap/imageset-large-4bpp.png");
	let mut set = VeraImageSet::new("tileset_error", 8, 8);
	let config = VeraImageSetLoadConfig::default();
	set.load_from_png(setdata.to_vec(), &config)?;
	set.format_indices(&palette, VeraPixelDepth::BPP4)?;
	println!("{}", set);

	let frame = set.frame_at(256)?;
	println!("{}", frame);

	// Init tilemap
	let mut tilemap = VeraTileMap::init_from_imageset(
		"tilemap_error",
		VeraTileMapMode::Tile4BPP,
		VeraTileMapDim::Dim64,
		VeraTileMapDim::Dim32,
		&set,
	)?;

	// And a correct one
	let mapdata = include_bytes!("data/tilemap/tilemap-large-4bpp.png");
	tilemap.load_from_png(mapdata.to_vec(), None, 0, 0, 0)?;

	// Check high bit set correctly
	let mut high_bit_index = 0;

	for (i, f) in tilemap.get_tiles().iter().enumerate() {
		if let VeraTileMapEntry::Tile234(281, _, _, _) = f {
			high_bit_index = i;
			break;
		}
	}

	let code = tilemap.assemble()?;
	println!("{}", tilemap);

	let low = code.data_raw()[high_bit_index * 2];
	let high = code.data_raw()[high_bit_index * 2 + 1];
	let mut test_u16: u16 = (high as u16 & 3) << 8;
	test_u16 |= low as u16;
	assert_eq!(281, test_u16);

	let asm = code.assemble_meta(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);

	let asm = code.assemble_data(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);

	Ok(())
}
