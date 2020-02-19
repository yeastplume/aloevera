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

use aloevera_vera::{VeraPalette, VeraPaletteEntry, VeraPaletteLoadConfig};

#[test]
fn palette_8bpp_rgb() -> Result<(), Error> {
	init_test_logger();
	let test_png = include_bytes!("data/palette/palette-8bpp-rgb.png");
	let pal_config = VeraPaletteLoadConfig {
		direct_load: true,
		include_defaults: false,
		sort: false,
	};
	let palette = VeraPalette::derive_from_png("pal", test_png.to_vec(), &pal_config)?;

	println!("{}", palette);
	assert_eq!(
		palette.value_at_index(0)?,
		VeraPaletteEntry { r: 0, g: 0, b: 0 }
	);
	assert_eq!(
		palette.value_at_index(16)?,
		VeraPaletteEntry { r: 0, g: 0, b: 0 }
	);
	assert_eq!(
		palette.value_at_index(17)?,
		VeraPaletteEntry {
			r: 0x5,
			g: 0x6,
			b: 0xe
		}
	);
	assert_eq!(palette.len(), 32);

	Ok(())
}

#[test]
fn palette_8bpp_indexed() -> Result<(), Error> {
	init_test_logger();
	let test_png = include_bytes!("data/palette/palette-8bpp-indexed.png");
	let pal_config = VeraPaletteLoadConfig {
		direct_load: true,
		include_defaults: false,
		sort: false,
	};
	let palette = VeraPalette::derive_from_png("pal", test_png.to_vec(), &pal_config)?;

	println!("{}", palette);
	assert_eq!(
		palette.value_at_index(0)?,
		VeraPaletteEntry { r: 0, g: 0, b: 0 }
	);
	assert_eq!(
		palette.value_at_index(16)?,
		VeraPaletteEntry { r: 0, g: 0, b: 0 }
	);
	assert_eq!(
		palette.value_at_index(17)?,
		VeraPaletteEntry {
			r: 0x5,
			g: 0x6,
			b: 0xe
		}
	);
	assert_eq!(palette.len(), 32);

	Ok(())
}
