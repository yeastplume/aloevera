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

const LOW_RAM_SIZE: usize = 38655;
const LOW_RAM_WARN_THRESHOLD: f64 = 0.9;

/// Arguments for asm commands
pub struct AsmArgs {
	pub out_dir: String,
	pub format: AsmFormat,
	pub sd_image: Option<String>,
	pub conflate_tilemaps: bool,
}

fn perform_assemble<T>(
	values: &mut dyn Iterator<Item = &T>,
	asm_args: &AsmArgs,
	sel_args: Option<&AsmSelectArgs>,
	line_start: &mut usize,
) -> Result<usize, Error>
where
	T: Assemblable,
{
	let mut assembled_size = 0;
	for v in values {
		let code = v.assemble()?;
		let conflate = asm_args.format == AsmFormat::Bin || asm_args.conflate_tilemaps;
		let asm_meta = code.assemble_meta(asm_args.format.clone(), conflate)?;
		let meta_lc = asm_meta.line_count();
		let (output, ext) = if asm_args.format == AsmFormat::Bin {
			let (file_name, bin_address) = match sel_args.clone() {
				Some(s) => (s.out_file.clone(), s.bin_address.clone()),
				None => (format!("{}/{}.bin", asm_args.out_dir, v.id()), [0, 0]),
			};
			common::output_to_file(
				&file_name,
				&code.data_as_bin(Some(bin_address), true)?,
				&asm_args.sd_image,
			)?;
			(asm_meta.to_string(None)?, "meta")
		} else {
			let asm_data = code.assemble_data(asm_args.format.clone(), conflate)?;
			let data_lc = asm_data.line_count();
			let output = asm_meta.to_string(Some(*line_start))?;
			let res = format!(
				"{}{}",
				output,
				asm_data.to_string(Some(*line_start + meta_lc))?
			);
			*line_start += meta_lc + data_lc;
			let mut ext = "inc";
			if asm_args.format == AsmFormat::Cc65 {
				ext = "h"
			}
			(res, ext)
		};
		let mut file_name = format!(
			"{}/{}.{}.{}",
			asm_args.out_dir,
			v.id(),
			asm_args.format,
			ext
		);
		if let Some(s) = sel_args.clone() {
			if asm_args.format == AsmFormat::Bin {
				file_name = format!("{}.meta", s.out_file);
			} else {
				file_name = s.out_file.clone();
			}
		}
		common::output_to_file(&file_name, output.as_bytes(), &asm_args.sd_image)?;
		let size = v.size_in_bytes(conflate)?;
		// Warn if we're getting close to t
		info!("Resource {} has size {}", v.id(), size);
		if size >= (LOW_RAM_SIZE as f64 * LOW_RAM_WARN_THRESHOLD) as usize
			&& asm_args.format != AsmFormat::Bin
		{
			warn!("Resource {} has a size of {} bytes, which approaches or exceeds the size of Low RAM ({}) bytes. Consider outputting as .BIN instead.", v.id(), size, LOW_RAM_SIZE);
		}
		assembled_size += v.size_in_bytes(conflate)?;
	}
	Ok(assembled_size)
}

/// Assemble
pub fn asm_all(g_args: &GlobalArgs, mut args: AsmArgs) -> Result<(), Error> {
	let proj = common::load_project(g_args.project_file.clone())?;
	// Todo: make this a flag?
	//common::remove_dir(&args.out_dir)?;
	let mut line_start = 10000;
	let start_dir = args.out_dir.clone();
	let mut tot_size = 0;
	// Output palettes
	if !proj.palettes.is_empty() {
		args.out_dir = format!("{}/palettes", start_dir);
		common::create_dir(&args.out_dir)?;
		tot_size += perform_assemble(&mut proj.palettes.values(), &args, None, &mut line_start)?;
	}
	if !proj.imagesets.is_empty() {
		args.out_dir = format!("{}/imagesets", start_dir);
		common::create_dir(&args.out_dir)?;
		tot_size += perform_assemble(&mut proj.imagesets.values(), &args, None, &mut line_start)?;
	}
	if !proj.tilemaps.is_empty() {
		args.out_dir = format!("{}/tilemaps", start_dir);
		common::create_dir(&args.out_dir)?;
		tot_size += perform_assemble(&mut proj.tilemaps.values(), &args, None, &mut line_start)?;
	}
	let mut sprites = vec![];
	if !proj.sprites.is_empty() {
		args.out_dir = format!("{}/sprites", start_dir);
		common::create_dir(&args.out_dir)?;
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
		// Don't include BMP size in total, since the imageset is already accounted for
		perform_assemble(&mut sprites.iter(), &args, None, &mut line_start)?;
	}
	let mut bitmaps = vec![];
	if !proj.bitmaps.is_empty() {
		args.out_dir = format!("{}/bitmaps", start_dir);
		common::create_dir(&args.out_dir)?;
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
		// Don't include BMP size in total, since the imageset is already accounted for
		perform_assemble(&mut bitmaps.iter(), &args, None, &mut line_start)?;
	}

	if tot_size >= (LOW_RAM_SIZE as f64 * LOW_RAM_WARN_THRESHOLD) as usize
		&& args.format != AsmFormat::Bin
	{
		warn!("Combined resources have a size of {} bytes, which approaches or exceeds the size of Low RAM ({}) bytes. Consider outputting as .BIN instead.", tot_size, LOW_RAM_SIZE);
	}

	Ok(())
}

/// Arguments for asm commands
pub struct AsmSelectArgs {
	pub asset_id: String,
	pub out_file: String,
	pub bin_address: [u8; 2],
}

pub fn asm_select(
	g_args: &GlobalArgs,
	asm_args: AsmArgs,
	args: &AsmSelectArgs,
) -> Result<(), Error> {
	let proj = common::load_project(g_args.project_file.clone())?;
	let mut line_start = 10000;
	common::create_dir(&asm_args.out_dir)?;
	// now look for the ID
	if proj.palettes.contains_key(&args.asset_id) {
		perform_assemble(
			&mut proj.palettes.values().filter(|v| v.id == args.asset_id),
			&asm_args,
			Some(args),
			&mut line_start,
		)?;
		return Ok(());
	}
	if proj.imagesets.contains_key(&args.asset_id) {
		perform_assemble(
			&mut proj.imagesets.values().filter(|v| v.id == args.asset_id),
			&asm_args,
			Some(&args),
			&mut line_start,
		)?;
		return Ok(());
	}
	if proj.tilemaps.contains_key(&args.asset_id) {
		perform_assemble(
			&mut proj.tilemaps.values().filter(|v| v.id == args.asset_id),
			&asm_args,
			Some(&args),
			&mut line_start,
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
			&asm_args,
			Some(&args),
			&mut line_start,
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
			&asm_args,
			Some(&args),
			&mut line_start,
		)?;
		return Ok(());
	}
	let msg = format!(
		"Asset with id {} does not exist in project file.",
		args.asset_id,
	);
	Err(ErrorKind::ArgumentError(msg).into())
}
