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

use crate::cmd::common::{self, GlobalArgs};
use crate::cmd::{asm, bitmap, create, imageset, palette, sprite, tilemap};
use crate::{Error, ErrorKind};

fn parse_and_execute(g_args: &GlobalArgs, args: &ArgMatches) -> Result<(), Error> {
	match args.subcommand() {
		("create", Some(args)) => create::parse::execute_create_command(&args),
		("asm", Some(args)) => asm::parse::execute_asm_command(g_args, &args),
		("palette", Some(args)) => palette::parse::execute_palette_command(g_args, &args),
		("sprite", Some(args)) => sprite::parse::execute_sprite_command(g_args, &args),
		("bitmap", Some(args)) => bitmap::parse::execute_bitmap_command(g_args, &args),
		("imageset", Some(args)) => imageset::parse::execute_imageset_command(g_args, &args),
		("tilemap", Some(args)) => tilemap::parse::execute_tilemap_command(g_args, &args),
		_ => {
			let msg = format!("Unknown command, use 'aloevera --help' for details");
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	}
}

pub fn execute_command(args: &ArgMatches) -> Result<String, Error> {
	let g_args = common::parse_global_args(&args)?;
	let res = parse_and_execute(&g_args, &args);
	if let Err(e) = res {
		Err(e)
	} else {
		Ok(args.subcommand().0.to_owned())
	}
}
