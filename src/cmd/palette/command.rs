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
use proj::{AloeVeraProject, Jsonable};

use crate::cmd::common::{self, GlobalArgs};
use vera::{VeraPalette, VeraPaletteLoadConfig};

/// Arguments for palette command
pub struct PaletteImportArgs {
	pub id: String,
	pub input_file: String,
}

/// Palette import command
pub fn palette_import(g_args: &GlobalArgs, args: &PaletteImportArgs) -> Result<(), Error> {
	let is_gpl = args.input_file.ends_with("gpl") || args.input_file.ends_with("GPL");
	let palette = match is_gpl {
		true => {
			let pal_config = VeraPaletteLoadConfig {
				direct_load: true,
				include_defaults: false,
				sort: false,
			};
			VeraPalette::derive_from_gpl(&args.id, &args.input_file, &pal_config).expect("Error")
		}
		false => {
			let png_bytes = common::read_file_bin(&args.input_file)?;
			let pal_config = VeraPaletteLoadConfig {
				direct_load: true,
				include_defaults: false,
				sort: false,
			};
			VeraPalette::derive_from_png(&args.id, png_bytes, &pal_config).expect("error")
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
	let proj_json = common::read_file_string(&project_file)?;
	let mut proj = AloeVeraProject::new_from_json(&proj_json)?;
	proj.palettes.insert(palette.id.clone(), palette);
	common::output_to_file(&project_file, &proj.to_json()?.as_bytes(), &None)?;

	Ok(())
}

/// Palette list command
pub fn palette_list(g_args: &GlobalArgs) -> Result<(), Error> {
	// load up the project json
	let project_file = match &g_args.project_file {
		Some(f) => f,
		None => {
			return Err(ErrorKind::ArgumentError("Missing project file name".to_string()).into())
		}
	};
	let proj_json: String = common::read_file_string(&project_file)?;
	let proj = AloeVeraProject::new_from_json(&proj_json)?;
	println!("Palettes:");
	for (id, palette) in proj.palettes {
		println!("  {}: {} colors", id, palette.len());
	}

	Ok(())
}
