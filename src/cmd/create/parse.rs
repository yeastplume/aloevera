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
use super::command::{self, CreateProjectArgs, CreateSDImageArgs};
use crate::cmd::common;
use crate::{Error, ErrorKind};
use clap::ArgMatches;

pub fn parse_create_project_args(args: &ArgMatches) -> Result<CreateProjectArgs, Error> {
	let output_file = common::parse_required(args, "output_file")?;
	Ok(CreateProjectArgs {
		id: args.value_of("id").map(|s| s.into()),
		output_file: output_file.into(),
	})
}

pub fn parse_create_sd_image_args(args: &ArgMatches) -> Result<CreateSDImageArgs, Error> {
	let output_file = common::parse_required(args, "output_file")?;
	Ok(CreateSDImageArgs {
		output_file: output_file.into(),
	})
}

pub fn execute_create_command(args: &ArgMatches) -> Result<(), Error> {
	match args.subcommand() {
		("project", Some(args)) => {
			let a = arg_parse!(parse_create_project_args(args));
			command::create_project(&a)
		}
		("sdimage", Some(args)) => {
			let a = arg_parse!(parse_create_sd_image_args(args));
			command::create_sd_image(&a)
		}
		_ => {
			let msg = format!("Unknown sub command, use 'aloevera create --help' for details");
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	}
}
