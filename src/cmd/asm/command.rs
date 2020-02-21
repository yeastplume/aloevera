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

use std::iter::Iterator;

use crate::cmd::common::{self, GlobalArgs};
use crate::{Error, ErrorKind};
use vera::{AsmFormat, Assemblable, VeraBitmap, VeraSprite};

/// Arguments for asm commands
pub struct AsmArgs {
	pub out_dir: String,
	pub format: AsmFormat,
	pub sd_image: Option<String>,
}

fn perform_assemble<T>(
	values: &mut dyn Iterator<Item = &T>,
	output_format: &AsmFormat,
	out_dir: &str,
	file_name: Option<&str>,
	line_start: &mut usize,
	sd_image: &Option<String>,
) -> Result<(), Error>
where
	T: Assemblable,
{
	for v in values {
		let code = v.assemble()?;
		let asm_meta = code.assemble_meta(output_format.clone())?;
		let meta_lc = asm_meta.line_count();
		let (output, ext) = if *output_format == AsmFormat::Bin {
			let file_name = match file_name {
				Some(f) => f.into(),
				None => format!("{}/{}.bin", out_dir, v.id()),
			};
			common::output_to_file(&file_name, &code.data_as_bin(None), sd_image)?;
			(asm_meta.to_string(None)?, "meta")
		} else {
			let asm_data = code.assemble_data(output_format.clone())?;
			let data_lc = asm_data.line_count();
			let output = asm_meta.to_string(Some(*line_start))?;
			let res = format!(
				"{}{}",
				output,
				asm_data.to_string(Some(*line_start + meta_lc))?
			);
			*line_start += meta_lc + data_lc;
			(res, "inc")
		};
		let file_name = match file_name {
			Some(f) => format!("{}.meta", f),
			None => format!("{}/{}.{}.{}", out_dir, v.id(), output_format, ext),
		};
		common::output_to_file(&file_name, output.as_bytes(), sd_image)?;
	}
	Ok(())
}

/// Assemble
pub fn asm_all(g_args: &GlobalArgs, args: &AsmArgs) -> Result<(), Error> {
	let proj = common::load_project(g_args.project_file.clone())?;
	// Todo: make this a flag?
	//common::remove_dir(&args.out_dir)?;
	let mut line_start = 10000;
	// Output palettes
	if !proj.palettes.is_empty() {
		let pal_dir = format!("{}/palettes", args.out_dir);
		common::create_dir(&pal_dir)?;
		perform_assemble(
			&mut proj.palettes.values(),
			&args.format,
			&pal_dir,
			None,
			&mut line_start,
			&args.sd_image,
		)?;
	}
	if !proj.imagesets.is_empty() {
		let img_dir = format!("{}/imagesets", args.out_dir);
		common::create_dir(&img_dir)?;
		perform_assemble(
			&mut proj.imagesets.values(),
			&args.format,
			&img_dir,
			None,
			&mut line_start,
			&args.sd_image,
		)?;
	}
	if !proj.tilemaps.is_empty() {
		let tm_dir = format!("{}/tilemaps", args.out_dir);
		common::create_dir(&tm_dir)?;
		perform_assemble(
			&mut proj.tilemaps.values(),
			&args.format,
			&tm_dir,
			None,
			&mut line_start,
			&args.sd_image,
		)?;
	}
	let mut sprites = vec![];
	if !proj.sprites.is_empty() {
		let sp_dir = format!("{}/sprites", args.out_dir);
		common::create_dir(&sp_dir)?;
		for s in proj.sprites.values() {
			//Repopulate references
			let imageset = match proj.imagesets.get(&s.imageset_id) {
				Some(i) => i,
				None => {
					let msg = format!(
						"Imageset with id {} needed by sprite {} does not exist in project file.",
						s.id, s.imageset_id
					);
					return Err(ErrorKind::ArgumentError(msg).into());
				}
			};
			let sprite = VeraSprite::init_from_imageset(&s.id, &imageset)?;
			sprites.push(sprite);
		}
		perform_assemble(
			&mut sprites.iter(),
			&args.format,
			&sp_dir,
			None,
			&mut line_start,
			&args.sd_image,
		)?;
	}
	let mut bitmaps = vec![];
	if !proj.bitmaps.is_empty() {
		let bm_dir = format!("{}/bitmaps", args.out_dir);
		common::create_dir(&bm_dir)?;
		for b in proj.bitmaps.values() {
			//Repopulate references
			let imageset = match proj.imagesets.get(&b.imageset_id) {
				Some(i) => i,
				None => {
					let msg = format!(
						"Imageset with id {} needed by bitmap {} does not exist in project file.",
						b.id, b.imageset_id
					);
					return Err(ErrorKind::ArgumentError(msg).into());
				}
			};
			let bitmap = VeraBitmap::init_from_imageset(&b.id, &imageset)?;
			bitmaps.push(bitmap);
		}
		perform_assemble(
			&mut bitmaps.iter(),
			&args.format,
			&bm_dir,
			None,
			&mut line_start,
			&args.sd_image,
		)?;
	}

	Ok(())
}

/// Arguments for asm commands
pub struct AsmSelectArgs {
	pub asset_id: String,
	pub out_file: String,
}

pub fn asm_select(
	g_args: &GlobalArgs,
	asm_args: &AsmArgs,
	args: &AsmSelectArgs,
) -> Result<(), Error> {
	let proj = common::load_project(g_args.project_file.clone())?;
	let mut line_start = 10000;
	// now look for the ID
	if proj.palettes.contains_key(&args.asset_id) {
		perform_assemble(
			&mut proj.palettes.values().filter(|v| v.id == args.asset_id),
			&asm_args.format,
			".",
			Some(&args.out_file),
			&mut line_start,
			&asm_args.sd_image,
		)?;
		return Ok(());
	}
	if proj.imagesets.contains_key(&args.asset_id) {
		perform_assemble(
			&mut proj.imagesets.values().filter(|v| v.id == args.asset_id),
			&asm_args.format,
			".",
			Some(&args.out_file),
			&mut line_start,
			&asm_args.sd_image,
		)?;
		return Ok(());
	}
	if proj.tilemaps.contains_key(&args.asset_id) {
		perform_assemble(
			&mut proj.tilemaps.values().filter(|v| v.id == args.asset_id),
			&asm_args.format,
			".",
			Some(&args.out_file),
			&mut line_start,
			&asm_args.sd_image,
		)?;
		return Ok(());
	}
	if proj.sprites.contains_key(&args.asset_id) {
		let sprite = proj.sprites.get(&args.asset_id).unwrap();
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
		let sprite = VeraSprite::init_from_imageset(&sprite.id, &imageset)?;
		perform_assemble(
			&mut [sprite].to_vec().iter(),
			&asm_args.format,
			".",
			Some(&args.out_file),
			&mut line_start,
			&asm_args.sd_image,
		)?;
		return Ok(());
	}
	if proj.bitmaps.contains_key(&args.asset_id) {
		let bitmap = proj.bitmaps.get(&args.asset_id).unwrap();
		let imageset = match proj.imagesets.get(&bitmap.imageset_id) {
			Some(i) => i,
			None => {
				let msg = format!(
					"Imageset with id {} needed by bitmap {} does not exist in project file.",
					bitmap.id, bitmap.imageset_id
				);
				return Err(ErrorKind::ArgumentError(msg).into());
			}
		};
		let bitmap = VeraBitmap::init_from_imageset(&bitmap.id, &imageset)?;
		perform_assemble(
			&mut [bitmap].to_vec().iter(),
			&asm_args.format,
			".",
			Some(&args.out_file),
			&mut line_start,
			&asm_args.sd_image,
		)?;
		return Ok(());
	}
	let msg = format!(
		"Asset with id {} does not exist in project file.",
		args.asset_id,
	);
	Err(ErrorKind::ArgumentError(msg).into())
}
