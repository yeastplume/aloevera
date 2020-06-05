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

use crate::{Error, ErrorKind};
use proj::{AloeVeraProject, Binable};

use crate::cmd::common::{self, GlobalArgs};
use vera::{VeraPalette, VeraPaletteLoadConfig};

/// Arguments for palette command
pub struct PaletteImportArgs {
	pub id: String,
	pub input_file: String,
}

#[derive(Debug)]
/// Supported palette file types
enum PaletteFileType {
	PNG,
	GPL,
}

fn vec_compare(va: &[u8], vb: &[u8]) -> bool {
	(va.len() >= vb.len()) && va.iter().zip(vb).all(|(a, b)| a == b)
}

fn determine_file_type(data: &Vec<u8>) -> Result<PaletteFileType, Error> {
	// 8-byte PNG file header as described here: https://en.wikipedia.org/wiki/Portable_Network_Graphics#File_header
	const PNG_BYTES: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
	// First line of Gimp gpl file excluding line ending: "GIMP Palette"
	const GPL_BYTES: [u8; 12] = [71, 73, 77, 80, 32, 80, 97, 108, 101, 116, 116, 101];

	if vec_compare(data, &PNG_BYTES) {
		return Ok(PaletteFileType::PNG);
	}
	if vec_compare(data, &GPL_BYTES) {
		return Ok(PaletteFileType::GPL);
	}
	return Err(ErrorKind::ArgumentError("Invalid palette file".to_string()).into());
}

/// Palette import command
pub fn palette_import(g_args: &GlobalArgs, args: &PaletteImportArgs) -> Result<(), Error> {
	let pal_bytes = common::read_file_bin(&args.input_file)?;
	let pal_config = VeraPaletteLoadConfig {
		direct_load: true,
		include_defaults: false,
		sort: false,
	};
	let file_type = determine_file_type(&pal_bytes);
	let palette = match file_type {
		Ok(PaletteFileType::GPL) => VeraPalette::derive_from_gpl(&args.id, pal_bytes, &pal_config)?,
		Ok(PaletteFileType::PNG) => VeraPalette::derive_from_png(&args.id, pal_bytes, &pal_config)?,
		Err(s) => {
			return Err(s);
		}
	};
	// load up the project json
	let project_file = match &g_args.project_file {
		Some(f) => f,
		None => {
			return Err(ErrorKind::ArgumentError("Missing project file name".to_string()).into())
		}
	};
	info!("Inserting palette into project: {}", project_file);
	let encoded = common::read_file_bin(&project_file)?;
	let mut proj = *AloeVeraProject::from_bin(&encoded)?;
	proj.palettes.insert(palette.id.clone(), palette);
	common::output_to_file(&project_file, &proj.to_bin()?, &None)?;

	Ok(())
}
