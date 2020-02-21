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
use proj::Jsonable;

use crate::cmd::common::{self, GlobalArgs};
use crate::{Error, ErrorKind};
use vera::{VeraTileMap, VeraTileMapDim, VeraTileMapMode};

fn insert_tilemap(
	project_file: Option<String>,
	id: &str,
	tilemap: &VeraTileMap,
) -> Result<(), Error> {
	let mut proj = crate::cmd::common::load_project(project_file.clone())?;
	proj.tilemaps.insert(id.into(), tilemap.clone());
	crate::cmd::common::output_to_file(&project_file.unwrap(), &proj.to_json()?.as_bytes(), &None)?;
	Ok(())
}

/// Arguments for init tilemap command
pub struct InitTileMapArgs {
	pub id: String,
	pub imageset_id: String,
	pub map_width: VeraTileMapDim,
	pub map_height: VeraTileMapDim,
	pub display_mode: VeraTileMapMode,
}

/// Tilemap import command
pub fn tilemap_init(g_args: &GlobalArgs, args: &InitTileMapArgs) -> Result<(), Error> {
	let proj = common::load_project(g_args.project_file.clone())?;
	let imageset = match proj.imagesets.get(&args.imageset_id) {
		Some(i) => i.clone(),
		None => {
			let msg = format!("Imageset with id `{}` not found", args.imageset_id);
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	};
	let tilemap = VeraTileMap::init_from_imageset(
		&args.id,
		args.display_mode,
		args.map_width,
		args.map_height,
		&imageset,
	)?;
	insert_tilemap(g_args.project_file.clone(), &args.id, &tilemap)?;

	Ok(())
}

/// Arguments for tilemap load command
pub struct LoadTileMapArgs {
	pub id: String,
	pub palette_id: Option<String>,
	pub input_file: String,
	pub start_x: u32,
	pub start_y: u32,
	pub clear_index: u32,
}

/// Tilemap load command
pub fn tilemap_load(g_args: &GlobalArgs, args: &LoadTileMapArgs) -> Result<(), Error> {
	let proj = common::load_project(g_args.project_file.clone())?;
	let palette = match args.palette_id.clone() {
		Some(p) => match proj.palettes.get(&p) {
			Some(pal) => Some(pal),
			None => {
				let msg = format!("Palette with id `{}` not found", p);
				return Err(ErrorKind::ArgumentError(msg).into());
			}
		},
		None => None,
	};
	let mut tilemap = match proj.tilemaps.get(&args.id) {
		Some(i) => i.clone(),
		None => {
			let msg = format!("Tilemap with id `{}` not found", args.id);
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	};
	let png_bytes = common::read_file_bin(&args.input_file)?;
	tilemap.load_from_png(
		png_bytes.to_vec(),
		palette,
		args.start_x,
		args.start_y,
		args.clear_index as u8,
	)?;
	insert_tilemap(g_args.project_file.clone(), &args.id, &tilemap)?;

	Ok(())
}
