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

use super::command::{self, SpriteInitArgs};
use crate::cmd::common::{self, GlobalArgs};
use crate::{Error, ErrorKind};

pub fn parse_sprite_init_args(
	g_args: &GlobalArgs,
	args: &ArgMatches,
) -> Result<SpriteInitArgs, Error> {
	if g_args.project_file.is_none() {
		let msg = format!("--project_file is required in this context");
		return Err(ErrorKind::ArgumentError(msg).into());
	}
	let id = common::parse_required(args, "id")?;
	let imageset_id = common::parse_required(args, "imageset_id")?;
	Ok(SpriteInitArgs {
		id: id.to_owned(),
		imageset_id: imageset_id.into(),
	})
}

pub fn execute_sprite_command(g_args: &GlobalArgs, args: &ArgMatches) -> Result<(), Error> {
	match args.subcommand() {
		("init", Some(args)) => {
			let a = arg_parse!(parse_sprite_init_args(g_args, args));
			command::sprite_init(g_args, &a)
		}
		("list", Some(_)) => command::sprite_list(g_args),
		_ => {
			let msg = format!("Unknown sub command, use 'aloevera sprite --help' for details");
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	}
}
