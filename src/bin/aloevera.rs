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

//! Main executable
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
use aloevera_util as util;

use std::thread;
use std::time::Duration;

use clap::App;
use log::Level;

use crate::util::init_logger;
use crate::util::logger::LoggingConfig;
use aloevera::cmd::execute;

/// include build information
pub mod built_info {
	include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// Output info strings to log
pub fn info_strings() -> (String, String) {
	(
		format!(
			"This is Aloevera version {}, built for {} by {}.",
			built_info::PKG_VERSION,
			//built_info::GIT_VERSION.map_or_else(|| "".to_owned(), |v| format!(" (git {})", v)),
			built_info::TARGET,
			built_info::RUSTC_VERSION,
		)
		.to_string(),
		format!(
			"Built with profile \"{}\", features \"{}\".",
			built_info::PROFILE,
			built_info::FEATURES_STR,
		)
		.to_string(),
	)
}

fn log_build_info() {
	let (basic_info, detailed_info) = info_strings();
	info!("{}", basic_info);
	debug!("{}", detailed_info);
}

fn main() {
	let exit_code = real_main();
	std::process::exit(exit_code);
}

fn real_main() -> i32 {
	let yml = load_yaml!("aloevera.yml");
	let args = App::from_yaml(yml)
		.version(built_info::PKG_VERSION)
		.get_matches();
	let logging_config = LoggingConfig {
		stdout_log_level: Level::Info,
		file_log_level: Level::Debug,
		..LoggingConfig::default()
	};
	init_logger(Some(logging_config), None);
	log_build_info();

	let res = execute::execute_command(&args);

	let retval = {
		if let Err(e) = res {
			error!("Command failed: {}", e);
			1
		} else {
			info!("Command '{}' completed successfully", args.subcommand().0);
			0
		}
	};

	info!("Finished");

	// we need to give log output a chance to catch up before exiting
	thread::sleep(Duration::from_millis(100));
	retval
}
