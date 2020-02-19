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

use crate::cmd::common::{self, GlobalArgs};
use crate::{Error, ErrorKind};
use vera::{AsmFormat, Assemblable, VeraBitmap, VeraSprite};

/// Arguments for asm command
pub struct AsmArgs {
	pub out_dir: String,
	pub format: AsmFormat,
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
		for p in proj.palettes.values() {
			let asm = p.assemble(&args.format, &mut line_start)?;
			common::output_to_file(&format!("{}/{}.{}.inc", pal_dir, p.id, args.format), &asm)?;
		}
	}
	if !proj.imagesets.is_empty() {
		let img_dir = format!("{}/imagesets", args.out_dir);
		common::create_dir(&img_dir)?;
		for i in proj.imagesets.values() {
			let asm = i.assemble(&args.format, &mut line_start)?;
			common::output_to_file(&format!("{}/{}.{}.inc", img_dir, i.id, args.format), &asm)?;
		}
	}
	if !proj.tilemaps.is_empty() {
		let tm_dir = format!("{}/tilemaps", args.out_dir);
		common::create_dir(&tm_dir)?;
		for i in proj.tilemaps.values() {
			let asm = i.assemble(&args.format, &mut line_start)?;
			common::output_to_file(&format!("{}/{}.{}.inc", tm_dir, i.id, args.format), &asm)?;
		}
	}
	if !proj.sprites.is_empty() {
		let tm_dir = format!("{}/sprites", args.out_dir);
		common::create_dir(&tm_dir)?;
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
			let asm = sprite.assemble(&args.format, &mut line_start)?;
			common::output_to_file(&format!("{}/{}.{}.inc", tm_dir, s.id, args.format), &asm)?;
		}
	}
	if !proj.bitmaps.is_empty() {
		let tm_dir = format!("{}/bitmaps", args.out_dir);
		common::create_dir(&tm_dir)?;
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
			let asm = bitmap.assemble(&args.format, &mut line_start)?;
			common::output_to_file(&format!("{}/{}.{}.inc", tm_dir, b.id, args.format), &asm)?;
		}
	}

	Ok(())
}
