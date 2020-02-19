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

use super::command::{self, InitTileMapArgs, LoadTileMapArgs};
use crate::cmd::common::{self, GlobalArgs};
use crate::{Error, ErrorKind};
use vera::{VeraTileMapDim, VeraTileMapMode};

pub fn parse_init_tilemap_args(
	g_args: &GlobalArgs,
	args: &ArgMatches,
) -> Result<InitTileMapArgs, Error> {
	if g_args.project_file.is_none() {
		let msg = format!("--project_file is required in this context");
		return Err(ErrorKind::ArgumentError(msg).into());
	}
	let id = common::parse_required(args, "id")?;
	let imageset_id = common::parse_required(args, "imageset_id")?;
	let v = common::parse_required(args, "map_width")?;
	let v = common::parse_u64(&v, "map_width")?;
	let map_width = VeraTileMapDim::from_u32(v as u32)?;
	let v = common::parse_required(args, "map_height")?;
	let v = common::parse_u64(&v, "map_height")?;
	let map_height = VeraTileMapDim::from_u32(v as u32)?;
	let v = common::parse_required(args, "display_mode")?;
	let display_mode = VeraTileMapMode::from_input(&v)?;

	Ok(InitTileMapArgs {
		id: id.into(),
		imageset_id: imageset_id.into(),
		map_width,
		map_height,
		display_mode,
	})
}

pub fn parse_load_tilemap_args(
	g_args: &GlobalArgs,
	args: &ArgMatches,
) -> Result<LoadTileMapArgs, Error> {
	if g_args.project_file.is_none() {
		let msg = format!("--project_file is required in this context");
		return Err(ErrorKind::ArgumentError(msg).into());
	}
	let id = common::parse_required(args, "id")?;
	let input_file = common::parse_required(args, "input_file")?;
	let v = common::parse_required(args, "start_x")?;
	let start_x = common::parse_u64(&v, "start_x")?;
	let v = common::parse_required(args, "start_y")?;
	let start_y = common::parse_u64(&v, "start_y")?;
	let v = common::parse_required(args, "clear_index")?;
	let clear_index = common::parse_u64(&v, "clear_index")?;
	let palette_id = match args.value_of("palette_id") {
		Some(i) => Some(i.into()),
		None => None,
	};

	Ok(LoadTileMapArgs {
		id: id.into(),
		palette_id,
		input_file: input_file.into(),
		start_x: start_x as u32,
		start_y: start_y as u32,
		clear_index: clear_index as u32,
	})
}
pub fn execute_tilemap_command(g_args: &GlobalArgs, args: &ArgMatches) -> Result<(), Error> {
	match args.subcommand() {
		("init", Some(args)) => {
			let a = arg_parse!(parse_init_tilemap_args(g_args, args));
			command::tilemap_init(g_args, &a)
		}
		("load", Some(args)) => {
			let a = arg_parse!(parse_load_tilemap_args(g_args, args));
			command::tilemap_load(g_args, &a)
		}
		_ => {
			let msg = format!("Unknown sub command, use 'aloevera tilemap --help' for details");
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	}
}
