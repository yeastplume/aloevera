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

use crate::{Error, ErrorKind};
use std::fs::{self, File};
use std::io::{Read, Write};

use proj::AloeVeraProject;

use clap::ArgMatches;
// define what to do on argument error

#[macro_export]
macro_rules! arg_parse {
	( $r:expr ) => {
		match $r {
			Ok(res) => res,
			Err(e) => {
				return Err(ErrorKind::ArgumentError(format!("{}", e)).into());
				}
			}
	};
}

// parses a required value, or throws error with message otherwise
pub fn parse_required<'a>(args: &'a ArgMatches, name: &str) -> Result<&'a str, Error> {
	let arg = args.value_of(name);
	match arg {
		Some(ar) => Ok(ar),
		None => {
			let msg = format!("Value for argument '{}' is required in this context", name,);
			Err(ErrorKind::ArgumentError(msg).into())
		}
	}
}

// parses a number, or throws error with message otherwise
pub fn parse_u64(arg: &str, name: &str) -> Result<u64, Error> {
	let val = arg.parse::<u64>();
	match val {
		Ok(v) => Ok(v),
		Err(e) => {
			let msg = format!("Could not parse {} as a whole number. e={}", name, e);
			Err(ErrorKind::ArgumentError(msg).into())
		}
	}
}

// As above, but optional
pub fn parse_u64_or_none(arg: Option<&str>) -> Option<u64> {
	let val = match arg {
		Some(a) => a.parse::<u64>(),
		None => return None,
	};
	match val {
		Ok(v) => Some(v),
		Err(_) => None,
	}
}

/// Arguments common to all commands
pub struct GlobalArgs {
	/// project file on which the command should operate
	pub project_file: Option<String>,
}

pub fn parse_global_args(args: &ArgMatches) -> Result<GlobalArgs, Error> {
	Ok(GlobalArgs {
		project_file: args.value_of("project_file").map(|s| s.into()),
	})
}

pub fn output_to_file(path: &str, data: &str) -> Result<(), Error> {
	let mut file = File::create(&path)?;
	file.write_all(data.as_bytes())?;
	Ok(())
}

pub fn read_file_bin(path: &str) -> Result<Vec<u8>, Error> {
	let mut file = File::open(&path)?;
	let mut content = vec![];
	file.read_to_end(&mut content)?;
	Ok(content)
}

pub fn read_file_string(path: &str) -> Result<String, Error> {
	let mut file = File::open(&path)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;
	Ok(content)
}

pub fn create_dir(path: &str) -> Result<(), Error> {
	fs::create_dir_all(path)?;
	Ok(())
}

pub fn remove_dir(path: &str) -> Result<(), Error> {
	let _ = fs::remove_dir_all(path);
	Ok(())
}

pub fn load_project<'a>(project_file: Option<String>) -> Result<AloeVeraProject<'a>, Error> {
	// load up the project json
	let project_file = match project_file {
		Some(f) => f,
		None => {
			return Err(ErrorKind::ArgumentError("Missing project file name".to_string()).into())
		}
	};
	let proj_json = read_file_string(&project_file)?;
	let res = AloeVeraProject::new_from_json(&proj_json)?;
	Ok(res)
}
