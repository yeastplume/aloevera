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

use aloevera::Error;

mod common;
use common::{clean_output_dir, compare_results, execute_command, setup};

#[test]
fn cmd_line_palette() -> Result<(), Error> {
	let test_dir = format!(
		"{}/{}",
		env!("CARGO_MANIFEST_DIR"),
		"target/test_output/cmd_line_palette"
	);
	setup(&test_dir);
	let project_file = format!("{}/testproject.av", test_dir);
	load_app!(app);

	// Create Project
	let arg_vec = vec!["aloevera", "create", "project", &project_file];
	execute_command(&app, arg_vec)?;

	// Import palette
	let input_file = input_file!("imageset-4bpp.png");
	println!("Input file: {}", input_file);
	let arg_vec = vec![
		"aloevera",
		"-p",
		&project_file,
		"palette",
		"import",
		"imageset-4bpp-pal",
		input_file,
	];
	execute_command(&app, arg_vec)?;

	// Output palette and check formats
	let arg_vec = vec![
		"aloevera",
		"-p",
		&project_file,
		"asm",
		&test_dir,
		"select",
		"imageset-4bpp-pal",
		"imageset-4bpp-pal.ca65",
	];
	execute_command(&app, arg_vec)?;
	compare_results(
		test_out!(test_dir, "imageset-4bpp-pal.ca65"),
		ref_file!("imageset-4bpp-pal.ca65"),
	)?;

	let arg_vec = vec![
		"aloevera",
		"-p",
		&project_file,
		"asm",
		"-f",
		"bin",
		&test_dir,
		"select",
		"imageset-4bpp-pal",
		"imageset-4bpp-pal.bin",
	];
	execute_command(&app, arg_vec)?;
	compare_results(
		test_out!(test_dir, "imageset-4bpp-pal.bin"),
		ref_file!("imageset-4bpp-pal.bin"),
	)?;
	compare_results(
		test_out!(test_dir, "imageset-4bpp-pal.bin.meta"),
		ref_file!("imageset-4bpp-pal.bin.meta"),
	)?;

	let arg_vec = vec![
		"aloevera",
		"-p",
		&project_file,
		"asm",
		"-f",
		"cc65",
		&test_dir,
		"select",
		"imageset-4bpp-pal",
		"imageset-4bpp-pal.cc65",
	];
	execute_command(&app, arg_vec)?;
	compare_results(
		test_out!(test_dir, "imageset-4bpp-pal.cc65"),
		ref_file!("imageset-4bpp-pal.cc65"),
	)?;

	clean_output_dir(&test_dir);
	Ok(())
}
