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

use std::convert::TryFrom;

use crate::{Error, ErrorKind};
use proj::{AloeVeraProject, Binable};

use crate::cmd::common::{self, GlobalArgs};

/// Arguments for palette command
pub struct ListArgs {
	pub object_type: ListObjectType,
}

#[derive(Debug)]
/// Supported Elements to List
pub enum ListObjectType {
	All,
	Palettes,
	Imagesets,
	Tilemaps,
	Sprites,
	Bitmaps,
}

impl TryFrom<&str> for ListObjectType {
	type Error = Error;
	fn try_from(input: &str) -> Result<Self, Self::Error> {
		let res = match input {
			"all" => ListObjectType::All,
			"palettes" => ListObjectType::Palettes,
			"imagesets" => ListObjectType::Imagesets,
			"tilemaps" => ListObjectType::Tilemaps,
			"sprites" => ListObjectType::Sprites,
			"bitmaps" => ListObjectType::Bitmaps,
			n => {
				return Err(ErrorKind::ArgumentError(format!("Invalid object type: {}", n)).into())
			}
		};
		Ok(res)
	}
}

/// Project file list command
pub fn list(g_args: &GlobalArgs, args: &ListArgs) -> Result<(), Error> {
	// load up the project json
	let project_file = match &g_args.project_file {
		Some(f) => f,
		None => {
			return Err(ErrorKind::ArgumentError("Missing project file name".to_string()).into())
		}
	};
	let encoded = common::read_file_bin(&project_file)?;
	let proj = *AloeVeraProject::from_bin(&encoded)?;
	println!("Elements in {}", project_file);
	println!("--------------");

	match args.object_type {
		ListObjectType::All => {
			list_palettes(&proj)?;
			list_imagesets(&proj)?;
			list_tilemaps(&proj)?;
			list_sprites(&proj)?;
			list_bitmaps(&proj)?;
		}
		ListObjectType::Palettes => {
			list_palettes(&proj)?;
		}
		ListObjectType::Imagesets => {
			list_imagesets(&proj)?;
		}
		ListObjectType::Tilemaps => {
			list_tilemaps(&proj)?;
		}
		ListObjectType::Sprites => {
			list_sprites(&proj)?;
		}
		ListObjectType::Bitmaps => {
			list_bitmaps(&proj)?;
		}
	}
	Ok(())
}

/// Palette list
fn list_palettes(proj: &AloeVeraProject) -> Result<(), Error> {
	println!("Palettes:");
	for (id, palette) in proj.palettes.iter() {
		println!("   {}:", id);
		println!("      Color Count: {}", palette.len());
	}
	Ok(())
}

/// Imageset list
pub fn list_imagesets(proj: &AloeVeraProject) -> Result<(), Error> {
	println!("Imagesets:");
	for (id, imageset) in proj.imagesets.iter() {
		let depth = match imageset.depth {
			Some(d) => format!("{}", d),
			None => "Unformatted".to_owned(),
		};
		println!("   {}:", id);
		println!("      Frame Count: {}", imageset.frame_data.len());
		println!(
			"      Frame Size: {}w x {}h",
			imageset.frame_width, imageset.frame_height,
		);
		println!("      Pixel Depth: {}", depth,);
	}
	Ok(())
}

/// Tilemap list
pub fn list_tilemaps(proj: &AloeVeraProject) -> Result<(), Error> {
	println!("Tilemaps:");
	for (id, tilemap) in proj.tilemaps.iter() {
		println!("   {}:", id);
		println!("      Using Imageset: {}", tilemap.imageset_id,);
		println!(
			"      Map Size: {}x{} Tiles",
			tilemap.map_width(),
			tilemap.map_height(),
		);
		println!(
			"      Tile Size: {}w x{}h",
			tilemap.tile_width(),
			tilemap.tile_height(),
		);
		println!("      Mode: {}", tilemap.mode,);
	}
	Ok(())
}

/// Sprite list
pub fn list_sprites(proj: &AloeVeraProject) -> Result<(), Error> {
	println!("Sprites:");
	for (id, sprite) in proj.sprites.iter() {
		let imageset = match proj.imagesets.get(&sprite.imageset_id) {
			Some(i) => i,
			None => {
				let msg = format!(
					"Imageset with id {} needed by sprite {} does not exist in project file.",
					sprite.id, sprite.imageset_id
				);
				return Err(ErrorKind::ArgumentError(msg).into());
			}
		};
		println!("   {}:", id);
		println!("      Using Imageset: {}", sprite.imageset_id,);
		println!("      Frame Count: {}", imageset.frame_data.len(),);
		println!(
			"      Frame Size: {}w x {}h",
			sprite.frame_width.val_as_u32(),
			sprite.frame_height.val_as_u32(),
		);
		println!("      Pixel Depth: {}", sprite.depth,);
	}
	Ok(())
}

/// Bitmap list
pub fn list_bitmaps(proj: &AloeVeraProject) -> Result<(), Error> {
	println!("Bitmaps:");
	for (id, bitmap) in proj.bitmaps.iter() {
		println!("   {}:", id);
		println!("      Using Imageset: {}", bitmap.imageset_id,);
		println!("      Width: {}", bitmap.width.val_as_u32(),);
		println!("      Pixel Depth: {}", bitmap.depth,);
	}
	Ok(())
}
