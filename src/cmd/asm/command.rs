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

/// Arguments for asm command
pub struct AsmArgs {
	pub out_dir: String,
	pub format: AsmFormat,
}

fn perform_assemble<T>(
	values: &mut dyn Iterator<Item = &T>,
	output_format: &AsmFormat,
	out_dir: &str,
	line_start: &mut usize,
) -> Result<(), Error>
where
	T: Assemblable,
{
	for v in values {
		let code = v.assemble()?;
		let asm_meta = code.assemble_meta(output_format.clone())?;
		let meta_lc = asm_meta.line_count();
		let (output, ext) = if *output_format == AsmFormat::Bin {
			let file_name = format!("{}/{}.bin", out_dir, v.id());
			common::output_to_file(&file_name, &code.data)?;
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
		let file_name = format!("{}/{}.{}.{}", out_dir, v.id(), output_format, ext);
		common::output_to_file(&file_name, output.as_bytes())?;
	}
	Ok(())
}

/// Assemble
pub fn asm(g_args: &GlobalArgs, args: &AsmArgs) -> Result<(), Error> {
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
			&mut line_start,
		)?;
	}
	if !proj.imagesets.is_empty() {
		let img_dir = format!("{}/imagesets", args.out_dir);
		common::create_dir(&img_dir)?;
		perform_assemble(
			&mut proj.imagesets.values(),
			&args.format,
			&img_dir,
			&mut line_start,
		)?;
	}
	if !proj.tilemaps.is_empty() {
		let tm_dir = format!("{}/tilemaps", args.out_dir);
		common::create_dir(&tm_dir)?;
		perform_assemble(
			&mut proj.tilemaps.values(),
			&args.format,
			&tm_dir,
			&mut line_start,
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
		perform_assemble(&mut sprites.iter(), &args.format, &sp_dir, &mut line_start)?;
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
		perform_assemble(&mut bitmaps.iter(), &args.format, &bm_dir, &mut line_start)?;
	}

	Ok(())
}
