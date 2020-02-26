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
};

#[test]
fn imageset_failures() -> Result<(), Error> {
	init_test_logger();
	// fail if indices don't exist in the palette
	let test_png = include_bytes!("data/imageset/rgba-16-1-x-1.png");
	let palette = VeraPalette::blank_with_defaults("blank");
	println!("{}", palette);

	let mut set = VeraImageSet::new("set_1", 16, 16);
	let config = VeraImageSetLoadConfig::default();
	set.load_from_png(test_png.to_vec(), &config)?;
	assert!(set.format_indices(&palette, VeraPixelDepth::BPP8).is_err());

	Ok(())
}

#[test]
fn imageset_rgba_16_1_x_1_8bpp() -> Result<(), Error> {
	init_test_logger();
	let test_png = include_bytes!("data/imageset/rgba-16-1-x-1.png");
	let pal_config = VeraPaletteLoadConfig::default();
	let palette = VeraPalette::derive_from_png("pal", test_png.to_vec(), &pal_config)?;

	println!("{}", palette);
	assert_eq!(palette.len(), 4);
	assert_eq!(palette.index_of_rgb(0x40, 0x60, 0x20), Some(1));
	assert_eq!(palette.index_of_rgb(0xB0, 0x10, 0x00), Some(2));

	let mut set = VeraImageSet::new("set_1", 16, 16);
	let config = VeraImageSetLoadConfig::default();
	set.load_from_png(test_png.to_vec(), &config)?;
	set.format_indices(&palette, VeraPixelDepth::BPP8)?;
	println!("{}", set);

	assert_eq!(set.size(), 256);
	assert!(set.frame_at(1).is_err());

	let frame = set.frame_at(0)?;
	println!("{}", frame);
	assert!(frame.pixel_at_coord(0, 16).is_err());
	assert!(frame.pixel_at_coord(16, 0).is_err());
	assert_eq!(frame.pixel_at_coord(0, 0)?.pal_index, Some(1));
	assert_eq!(frame.pixel_at_coord(2, 2)?.pal_index, Some(1));
	assert_eq!(frame.pixel_at_coord(15, 0)?.pal_index, Some(0));
	assert_eq!(frame.pixel_at_coord(14, 1)?.pal_index, Some(0));
	assert_eq!(frame.pixel_at_coord(7, 7)?.pal_index, Some(2));
	assert_eq!(frame.pixel_at_coord(8, 8)?.pal_index, Some(2));
	assert_eq!(frame.pixel_at_coord(14, 14)?.pal_index, Some(1));
	assert_eq!(frame.pixel_at_coord(15, 15)?.pal_index, Some(1));

	// Should be formattable to 4BPP
	set.format_indices(&palette, VeraPixelDepth::BPP4)?;

	// And 2BPP
	set.format_indices(&palette, VeraPixelDepth::BPP2)?;

	// 1 BPP just sets on or off for each pixel and ignores palette
	set.format_indices(&palette, VeraPixelDepth::BPP1)?;

	Ok(())
}

#[test]
fn imageset_rgba_16_4_x_4_8bpp() -> Result<(), Error> {
	init_test_logger();
	let test_png = include_bytes!("data/imageset/rgba-16-4-x-4.png");
	let pal_config = VeraPaletteLoadConfig::default();

	let palette = VeraPalette::derive_from_png("pal", test_png.to_vec(), &pal_config)?;
	println!("{}", palette);
	assert_eq!(palette.len(), 6);
	assert_eq!(palette.index_of_rgb(0x40, 0x60, 0x20), Some(1));
	assert_eq!(palette.index_of_rgb(0x50, 0x60, 0xE0), Some(2));
	assert_eq!(palette.index_of_rgb(0xB0, 0x10, 0x00), Some(4));

	let mut set = VeraImageSet::new("imageset 1", 16, 16);
	let mut config = VeraImageSetLoadConfig::default();
	set.load_from_png(test_png.to_vec(), &config)?;
	set.format_indices(&palette, VeraPixelDepth::BPP8)?;
	println!("{}", set);

	// 7 unique frames (including blank)
	assert_eq!(set.size(), 256 * 7);
	assert!(set.frame_at(7).is_err());
	assert!(set.frame_at_coord(0, 0).is_err());

	let frame = set.frame_at(0)?;
	println!("{}", frame);
	assert!(frame.pixel_at_coord(0, 16).is_err());
	assert!(frame.pixel_at_coord(16, 0).is_err());
	assert_eq!(frame.pixel_at_coord(0, 0)?.pal_index, Some(1));
	assert_eq!(frame.pixel_at_coord(2, 2)?.pal_index, Some(1));
	assert_eq!(frame.pixel_at_coord(15, 0)?.pal_index, Some(0));
	assert_eq!(frame.pixel_at_coord(14, 1)?.pal_index, Some(0));
	assert_eq!(frame.pixel_at_coord(7, 7)?.pal_index, Some(4));
	assert_eq!(frame.pixel_at_coord(8, 8)?.pal_index, Some(4));
	assert_eq!(frame.pixel_at_coord(14, 14)?.pal_index, Some(1));
	assert_eq!(frame.pixel_at_coord(15, 15)?.pal_index, Some(1));

	let frame = set.frame_at(5)?;
	println!("{}", frame);
	assert_eq!(frame.pixel_at_coord(0, 0)?.pal_index, Some(3));
	assert_eq!(frame.pixel_at_coord(2, 2)?.pal_index, Some(3));
	assert_eq!(frame.pixel_at_coord(15, 0)?.pal_index, Some(3));
	assert_eq!(frame.pixel_at_coord(14, 1)?.pal_index, Some(3));
	assert_eq!(frame.pixel_at_coord(7, 7)?.pal_index, Some(0));
	assert_eq!(frame.pixel_at_coord(8, 8)?.pal_index, Some(0));
	assert_eq!(frame.pixel_at_coord(14, 14)?.pal_index, Some(3));
	assert_eq!(frame.pixel_at_coord(15, 15)?.pal_index, Some(3));

	let frame = set.frame_at(1)?;
	println!("{}", frame);
	assert_eq!(frame.pixel_at_coord(15, 0)?.pal_index, Some(1));
	assert_eq!(frame.pixel_at_coord(14, 1)?.pal_index, Some(1));
	assert_eq!(frame.pixel_at_coord(14, 14)?.pal_index, Some(0));
	assert_eq!(frame.pixel_at_coord(15, 15)?.pal_index, Some(0));

	// quick test of non-culled mode
	config.cull_duplicates = false;
	set.load_from_png(test_png.to_vec(), &config)?;
	set.format_indices(&palette, VeraPixelDepth::BPP8)?;
	println!("{}", set);

	// 7 unique frame (including blank)
	assert_eq!(set.size(), 256 * 16);
	assert!(set.frame_at(16).is_err());
	let frame = set.frame_at_coord(2, 2)?;
	println!("{}", frame);
	assert_eq!(frame.pixel_at_coord(0, 0)?.pal_index, Some(3));
	assert_eq!(frame.pixel_at_coord(2, 2)?.pal_index, Some(3));
	assert_eq!(frame.pixel_at_coord(15, 0)?.pal_index, Some(3));
	assert_eq!(frame.pixel_at_coord(14, 1)?.pal_index, Some(3));
	assert_eq!(frame.pixel_at_coord(7, 7)?.pal_index, Some(0));
	assert_eq!(frame.pixel_at_coord(8, 8)?.pal_index, Some(0));
	assert_eq!(frame.pixel_at_coord(14, 14)?.pal_index, Some(3));
	assert_eq!(frame.pixel_at_coord(15, 15)?.pal_index, Some(3));

	Ok(())
}

#[test]
fn imageset_indexed_8_1_x_8_4bpp() -> Result<(), Error> {
	init_test_logger();
	// should fit exactly into an 8bpp imageset
	// since we're targeting 4bpp, import indexed straight as is
	let test_png = include_bytes!("data/imageset/indexed-8-1-x-8-4bpp.png");
	let pal_config = VeraPaletteLoadConfig {
		direct_load: true,
		include_defaults: false,
		sort: false,
		..VeraPaletteLoadConfig::default()
	};
	let palette = VeraPalette::derive_from_png("pal", test_png.to_vec(), &pal_config)?;

	println!("{}", palette);
	let mut set = VeraImageSet::new("set_1", 8, 8);
	let config = VeraImageSetLoadConfig::default();
	set.load_from_png(test_png.to_vec(), &config)?;
	set.format_indices(&palette, VeraPixelDepth::BPP4)?;
	println!("{}", set);

	let frame = set.frame_at(0)?;
	println!("{}", frame);
	assert_eq!(frame.pal_offset, 0);
	assert_eq!(frame.pixel_at_coord(3, 3)?.pal_index, Some(4));

	let frame = set.frame_at(1)?;
	println!("{}", frame);
	assert_eq!(frame.pal_offset, 16);
	assert_eq!(frame.pixel_at_coord(3, 3)?.pal_index, Some(4));

	assert_eq!(set.size(), 8 * 8 * 4 / 8 * 2);
	assert!(set.frame_at(2).is_err());

	// Won't be formattable to 2BPP
	assert!(set.format_indices(&palette, VeraPixelDepth::BPP2).is_err());

	Ok(())
}

#[test]
fn imageset_indexed_8_2_x_8_2bpp() -> Result<(), Error> {
	// 4 colors per frame max, and image palette has several
	// duplicates that need to be resolved in creating the
	// palette offsets
	init_test_logger();
	let test_png = include_bytes!("data/imageset/indexed-8-2-x-8-2bpp.png");
	let pal_config = VeraPaletteLoadConfig {
		direct_load: true,
		include_defaults: false,
		sort: false,
		..VeraPaletteLoadConfig::default()
	};
	let palette = VeraPalette::derive_from_png("pal", test_png.to_vec(), &pal_config)?;

	println!("{}", palette);
	let mut set = VeraImageSet::new("set_1", 8, 8);
	let config = VeraImageSetLoadConfig::default();
	set.load_from_png(test_png.to_vec(), &config)?;
	set.format_indices(&palette, VeraPixelDepth::BPP2)?;
	println!("{}", set);

	let frame = set.frame_at(0)?;
	println!("{}", frame);
	assert_eq!(frame.pal_offset, 0);
	assert_eq!(frame.pixel_at_coord(1, 1)?.pal_index, Some(0));

	let frame = set.frame_at(1)?;
	println!("{}", frame);
	assert_eq!(frame.pal_offset, 16);
	assert_eq!(frame.pixel_at_coord(7, 7)?.pal_index, Some(3));

	assert_eq!(set.size(), 8 * 8 * 2 / 8 * 2);
	assert!(set.frame_at(2).is_err());

	// BPP1 ignores palette
	set.format_indices(&palette, VeraPixelDepth::BPP1)?;

	// assemble 8 BPP
	set.format_indices(&palette, VeraPixelDepth::BPP8)?;
	println!("{}", set);
	let code = set.assemble()?;
	let asm = code.assemble_meta(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);
	let asm = code.assemble_data(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);

	// assemble 4 BPP
	set.format_indices(&palette, VeraPixelDepth::BPP4)?;
	println!("{}", set);
	let code = set.assemble()?;
	let asm = code.assemble_meta(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);
	let asm = code.assemble_data(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);

	// assemble 2 BPP
	set.format_indices(&palette, VeraPixelDepth::BPP2)?;
	println!("{}", set);
	let code = set.assemble()?;
	let asm = code.assemble_meta(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);
	let asm = code.assemble_data(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);

	Ok(())
}

#[test]
fn imageset_text_8_x_8_1bpp() -> Result<(), Error> {
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
	assert_eq!(frame.pal_offset, 0);
	assert_eq!(frame.pixel_at_coord(3, 1)?.is_on, true);

	println!("depth: {:?}", set.depth);
	let code = set.assemble()?;
	let asm = code.assemble_meta(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);
	let asm = code.assemble_data(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);

	// assemble BASIC
	let line_start = 1000;
	let asm = code.assemble_meta(crate::AsmFormat::Basic, false)?;
	let len_to_add = asm.line_count();
	println!("{}", asm.to_string(Some(line_start))?);
	let asm = code.assemble_data(crate::AsmFormat::Basic, false)?;
	println!("{}", asm.to_string(Some(line_start + len_to_add))?);

	Ok(())
}

#[test]
fn imageset_pal_64_4bpp() -> Result<(), Error> {
	// 4 BPP Palette with entries from the entire range
	// test that the palette offset (a multiple of 16) is applied correctly
	// in all frames
	init_test_logger();
	// since we're targeting 4bpp, import indexed straight as is
	let test_png = include_bytes!("data/imageset/indexed-4bpp-pal-64.png");
	let pal_config = VeraPaletteLoadConfig {
		direct_load: true,
		include_defaults: false,
		sort: false,
		..VeraPaletteLoadConfig::default()
	};
	let palette = VeraPalette::derive_from_png("pal", test_png.to_vec(), &pal_config)?;

	println!("{}", palette);
	let mut set = VeraImageSet::new("imageset_1", 8, 8);
	let config = VeraImageSetLoadConfig::default();
	set.load_from_png(test_png.to_vec(), &config)?;
	set.format_indices(&palette, VeraPixelDepth::BPP4)?;
	println!("{}", set);

	let frame = set.frame_at(1)?;
	println!("{}", frame);
	assert_eq!(frame.pal_offset, 16);
	assert_eq!(frame.pixel_at_coord(1, 1)?.pal_index, Some(10));

	Ok(())
}

/*#[test]
fn imageset_crash() -> Result<(), Error> {
	let test_png = include_bytes!("data/imageset/crash-test-pal.png");
	let pal_config = VeraPaletteLoadConfig {
		direct_load: true,
		include_defaults: false,
		sort: false,
		..VeraPaletteLoadConfig::default()
	};
	let palette = VeraPalette::derive_from_png("pal", test_png.to_vec(), &pal_config)?;
	println!("{}", palette);

	let imageset_png = include_bytes!("data/imageset/crash-test-imageset.png");
	let mut set = VeraImageSet::new("imageset_1", 16, 16);
	let config = VeraImageSetLoadConfig::default();
	set.load_from_png(imageset_png.to_vec(), &config)?;
	set.format_indices(&palette, VeraPixelDepth::BPP4)?;
	println!("{}", set);

	Ok(())

}*/
