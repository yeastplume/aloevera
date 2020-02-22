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
	VeraBitmap, VeraImageSet, VeraImageSetLoadConfig, VeraPalette, VeraPaletteLoadConfig,
	VeraPixelDepth,
};

#[test]
fn bitmap_load_8bpp() -> Result<(), Error> {
	init_test_logger();
	let test_png = include_bytes!("data/bitmap/bitmap-320x200-8bpp.png");
	let pal_config = VeraPaletteLoadConfig::default();
	let palette = VeraPalette::derive_from_png("pal", test_png.to_vec(), &pal_config)?;

	println!("{}", palette);

	let mut set = VeraImageSet::new("bitmap_imageset", 320, 200);
	let config = VeraImageSetLoadConfig::default();
	set.load_from_png(test_png.to_vec(), &config)?;
	set.format_indices(&palette, VeraPixelDepth::BPP8)?;
	println!("{}", set);

	// Now init the bitmap itself, which is just a bounds check
	let sprite = VeraBitmap::init_from_imageset("bitmap", &set)?;

	let code = sprite.assemble()?;
	let asm = code.assemble_meta(crate::AsmFormat::Ca65)?;
	println!("{}", asm.to_string(None)?);
	let asm = code.assemble_data(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(None)?);

	// assemble BASIC
	let line_start = 1000;
	let asm = code.assemble_meta(crate::AsmFormat::Basic)?;
	let len_to_add = asm.line_count();
	println!("{}", asm.to_string(Some(line_start))?);
	let asm = code.assemble_data(crate::AsmFormat::Ca65, false)?;
	println!("{}", asm.to_string(Some(line_start + len_to_add))?);

	Ok(())
}
