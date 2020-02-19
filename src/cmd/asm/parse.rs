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
use std::str::FromStr;

use clap::ArgMatches;

use super::command::AsmArgs;
use crate::cmd::common::{self, GlobalArgs};
use crate::{Error, ErrorKind};

use vera::AsmFormat;

pub fn parse_asm_args(g_args: &GlobalArgs, args: &ArgMatches) -> Result<AsmArgs, Error> {
	if g_args.project_file.is_none() {
		let msg = format!("--project_file is required in this context");
		return Err(ErrorKind::ArgumentError(msg).into());
	}
	let out_dir = common::parse_required(args, "out_dir")?;
	let asm_format = common::parse_required(args, "format")?;
	Ok(AsmArgs {
		out_dir: out_dir.into(),
		format: AsmFormat::from_str(asm_format)?,
	})
}
