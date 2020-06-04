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

//! Test wallet command line works as expected
#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

use aloevera::Error;

mod common;
use common::{clean_output_dir, execute_command, setup};

#[test]
fn command_line_basic() -> Result<(), Error> {
	let test_dir = "target/test_output/command_line_basic";
	setup(test_dir);
	let project_file = format!("{}/testproject.av", test_dir);
	load_app!(app);

	// Create Project
	let arg_vec = vec!["aloevera", "create", "project", &project_file];
	execute_command(&app, arg_vec)?;

	// Import palette
	let arg_vec = vec![
		"aloevera",
		"-p",
		&project_file,
		"palette",
		"import",
		"tile_wall_pal",
		"tests/data/input/imageset-4bpp.png",
	];
	execute_command(&app, arg_vec)?;

	// Output palette and check formats
	let arg_vec = vec![
		"aloevera",
		"-p",
		&project_file,
		"asm",
		"-f",
		"bin",
		"tests/data/input/imageset-4bpp.png",
	];
	execute_command(&app, arg_vec)?;

	clean_output_dir(test_dir);
	Ok(())
}
