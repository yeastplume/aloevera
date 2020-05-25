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

use clap::ArgMatches;

use super::command::{self, ImageSetFormatArgs, ImageSetImportArgs};
use crate::cmd::common::{self, GlobalArgs};
use crate::{Error, ErrorKind};
use vera::VeraPixelDepth;

pub fn parse_imageset_import_args(
	g_args: &GlobalArgs,
	args: &ArgMatches,
) -> Result<ImageSetImportArgs, Error> {
	if g_args.project_file.is_none() {
		let msg = format!("--project_file is required in this context");
		return Err(ErrorKind::ArgumentError(msg).into());
	}
	let input_file = common::parse_required(args, "input_file")?;
	let v = common::parse_required(args, "frame_width")?;
	let frame_width = common::parse_u64(&v, "frame_width")?;
	let v = common::parse_required(args, "frame_height")?;
	let frame_height = common::parse_u64(&v, "frame_height")?;
	let id = common::parse_required(args, "id")?;
	Ok(ImageSetImportArgs {
		id: id.into(),
		frame_height: frame_height as u32,
		frame_width: frame_width as u32,
		input_file: input_file.into(),
	})
}

pub fn parse_imageset_format_args(
	g_args: &GlobalArgs,
	args: &ArgMatches,
) -> Result<ImageSetFormatArgs, Error> {
	if g_args.project_file.is_none() {
		let msg = format!("--project_file is required in this context");
		return Err(ErrorKind::ArgumentError(msg).into());
	}
	let imageset_id = common::parse_required(args, "imageset_id")?;
	let palette_id = common::parse_required(args, "palette_id")?;
	let v = common::parse_required(args, "pixel_depth")?;
	let pixel_depth = common::parse_u64(&v, "pixel_depth")?;
	let pixel_depth = match pixel_depth {
		8 => VeraPixelDepth::BPP8,
		4 => VeraPixelDepth::BPP4,
		2 => VeraPixelDepth::BPP2,
		1 => VeraPixelDepth::BPP1,
		_ => {
			let msg = format!("Given pixel depth must be 1, 2, 4 or 8");
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	};

	Ok(ImageSetFormatArgs {
		imageset_id: imageset_id.into(),
		palette_id: palette_id.into(),
		pixel_depth,
	})
}

pub fn execute_imageset_command(g_args: &GlobalArgs, args: &ArgMatches) -> Result<(), Error> {
	match args.subcommand() {
		("import", Some(args)) => {
			let a = arg_parse!(parse_imageset_import_args(g_args, args));
			command::imageset_import(g_args, &a)
		}
		("format", Some(args)) => {
			let a = arg_parse!(parse_imageset_format_args(g_args, args));
			command::imageset_format(g_args, &a)
		}
		("list", Some(_)) => command::imageset_list(g_args),
		_ => {
			let msg = format!("Unknown sub command, use 'aloevera imageset --help' for details");
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	}
}
