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

use super::command::{self, PaletteImportArgs};
use crate::cmd::common::{self, GlobalArgs};
use crate::{Error, ErrorKind};

pub fn parse_palette_import_args(
	g_args: &GlobalArgs,
	args: &ArgMatches,
) -> Result<PaletteImportArgs, Error> {
	if g_args.project_file.is_none() {
		let msg = format!("--project_file is required in this context");
		return Err(ErrorKind::ArgumentError(msg).into());
	}
	let id = common::parse_required(args, "id")?;
	let input_file = common::parse_required(args, "input_file")?;
	Ok(PaletteImportArgs {
		id: id.to_owned(),
		input_file: input_file.into(),
	})
}

pub fn execute_palette_command(g_args: &GlobalArgs, args: &ArgMatches) -> Result<(), Error> {
	match args.subcommand() {
		("import", Some(args)) => {
			let a = arg_parse!(parse_palette_import_args(g_args, args));
			command::palette_import(g_args, &a)
		}
		("list", Some(_)) => command::palette_list(g_args),
		_ => {
			let msg = format!("Unknown sub command, use 'aloevera palette --help' for details");
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	}
}
