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
use proj::Jsonable;

use crate::cmd::common::{self, GlobalArgs};
use vera::{VeraImageSet, VeraImageSetLoadConfig, VeraPixelDepth};

fn insert_imageset(
	project_file: Option<String>,
	id: &str,
	imageset: &VeraImageSet,
) -> Result<(), Error> {
	//info!("Inserting imageset into project: {}", project_file);
	let mut proj = crate::cmd::common::load_project(project_file.clone())?;
	proj.imagesets.insert(id.into(), imageset.clone());
	crate::cmd::common::output_to_file(&project_file.unwrap(), &proj.to_json()?.as_bytes(), &None)?;
	Ok(())
}

/// Arguments for imageset import command
pub struct ImageSetImportArgs {
	pub id: String,
	pub input_file: String,
	pub frame_width: u32,
	pub frame_height: u32,
}

/// Imageset import command
pub fn imageset_import(g_args: &GlobalArgs, args: &ImageSetImportArgs) -> Result<(), Error> {
	let png_bytes = common::read_file_bin(&args.input_file)?;
	let config = VeraImageSetLoadConfig::default();
	println!("{}, {}", args.frame_width, args.frame_height);
	let mut imageset = VeraImageSet::new(&args.id, args.frame_width, args.frame_height);
	imageset.load_from_png(png_bytes, &config)?;
	insert_imageset(g_args.project_file.clone(), &args.id, &imageset)?;

	Ok(())
}

/// Arguments for imageset format command
pub struct ImageSetFormatArgs {
	pub imageset_id: String,
	pub palette_id: String,
	pub pixel_depth: VeraPixelDepth,
}

/// Imageset format
pub fn imageset_format(g_args: &GlobalArgs, args: &ImageSetFormatArgs) -> Result<(), Error> {
	let proj = common::load_project(g_args.project_file.clone())?;
	let palette = match proj.palettes.get(&args.palette_id) {
		Some(p) => p,
		None => {
			let msg = format!("Palette with id `{}` not found", args.palette_id);
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	};
	let mut imageset = match proj.imagesets.get(&args.imageset_id) {
		Some(i) => i.clone(),
		None => {
			let msg = format!("Imageset with id `{}` not found", args.imageset_id);
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	};
	imageset.format_indices(&palette, args.pixel_depth)?;
	insert_imageset(g_args.project_file.clone(), &args.imageset_id, &imageset)?;

	Ok(())
}

/// Imageset list
pub fn imageset_list(g_args: &GlobalArgs) -> Result<(), Error> {
	let proj = common::load_project(g_args.project_file.clone())?;
	println!("Image sets:");
	for (id, imageset) in proj.imagesets {
		match imageset.depth {
			None => println!(
				"  {}: {} {}x{} frames",
				id,
				imageset.frame_data.len(),
				imageset.frame_width,
				imageset.frame_height
			),
			pixel_depth => print!(
				"  {}: {} {}x{} frames depth {}",
				id,
				imageset.frame_data.len(),
				imageset.frame_width,
				imageset.frame_height,
				pixel_depth.unwrap()
			),
		}
	}
	Ok(())
}
