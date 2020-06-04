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

use clap::ArgMatches;

use super::command::{self, ListArgs, ListObjectType};
use crate::cmd::common::GlobalArgs;
use crate::{Error, ErrorKind};

pub fn parse_list_args(g_args: &GlobalArgs, args: &ArgMatches) -> Result<ListArgs, Error> {
	if g_args.project_file.is_none() {
		let msg = format!("--project_file is required in this context");
		return Err(ErrorKind::ArgumentError(msg).into());
	}
	let object_type = match args.value_of("object_type") {
		Some(v) => ListObjectType::try_from(v)?,
		None => ListObjectType::All,
	};
	Ok(ListArgs { object_type })
}

pub fn execute_list_command(g_args: &GlobalArgs, args: &ArgMatches) -> Result<(), Error> {
	let a = arg_parse!(parse_list_args(g_args, args));
	command::list(g_args, &a)
}
