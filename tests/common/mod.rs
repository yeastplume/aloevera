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

//! Common functions for integration tests
use clap::App;
use std::fs;

use aloevera::cmd;
use aloevera::Error;
use aloevera_util as util;

pub fn clean_output_dir(test_dir: &str) {
	let _ = fs::remove_dir_all(test_dir);
}

pub fn setup(test_dir: &str) {
	util::init_test_logger();
	clean_output_dir(test_dir);
	let _ = fs::create_dir_all(test_dir);
}

pub fn execute_command(app: &App, arg_vec: Vec<&str>) -> Result<String, Error> {
	let args = app.clone().get_matches_from(arg_vec);
	cmd::execute::execute_command(&args)
}

#[macro_export]
macro_rules! load_app {
	($app: ident) => {
		let yml = load_yaml!("../src/bin/aloevera.yml");
		let $app = clap::App::from_yaml(yml);
	};
}

#[macro_export]
macro_rules! test_out {
	($test_dir: ident, $filename: expr) => {
		&format!("{}/{}", $test_dir, $filename)
	};
}

#[macro_export]
macro_rules! input_file {
	($filename: expr) => {
		&format!(
			"{}/tests/data/input/{}",
			env!("CARGO_MANIFEST_DIR"),
			$filename
			)
	};
}

#[macro_export]
macro_rules! ref_file {
	($filename: expr) => {
		&format!(
			"{}/tests/data/ref/{}",
			env!("CARGO_MANIFEST_DIR"),
			$filename
			)
	};
}

pub fn compare_results(output_file: &str, reference_file: &str) -> Result<(), Error> {
	let output = cmd::common::read_file_bin(output_file)?;
	let reference = cmd::common::read_file_bin(reference_file)?;
	assert_eq!(output, reference);
	Ok(())
}
