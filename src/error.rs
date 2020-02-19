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

//! Error types for crate

use std::env;
use std::fmt::{self, Display};
use std::io;
use std::num::ParseIntError;

use failure::{Backtrace, Context, Fail};

/// Error definition
#[derive(Debug, Fail)]
pub struct Error {
	inner: Context<ErrorKind>,
}

/// Wallet errors, mostly wrappers around underlying crypto or I/O errors.
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
	/// Vera Error
	#[fail(display = "Vera module error: {}", _0)]
	Vera(vera::ErrorKind),
	/// Project Error
	#[fail(display = "Project Error")]
	Proj(proj::ErrorKind),
	/// IO Error
	#[fail(display = "I/O error: {}", _0)]
	IO(String),
	/// Parse Int error
	#[fail(display = "Parse Int error: {}", _0)]
	ParseIntError(ParseIntError),
	/// Argument Error
	#[fail(display = "Argument Error: {}", _0)]
	ArgumentError(String),
	/// Other
	#[fail(display = "Generic error: {}", _0)]
	GenericError(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let show_bt = match env::var("RUST_BACKTRACE") {
			Ok(r) => {
				if r == "1" {
					true
				} else {
					false
				}
			}
			Err(_) => false,
		};
		let backtrace = match self.backtrace() {
			Some(b) => format!("{}", b),
			None => String::from("Unknown"),
		};
		let inner_output = format!("{}", self.inner,);
		let backtrace_output = format!("\n Backtrace: {}", backtrace);
		let mut output = inner_output.clone();
		if show_bt {
			output.push_str(&backtrace_output);
		}
		Display::fmt(&output, f)
	}
}

impl Error {
	/// get kind
	pub fn kind(&self) -> ErrorKind {
		self.inner.get_context().clone()
	}
	/// get cause string
	pub fn cause_string(&self) -> String {
		match self.cause() {
			Some(k) => format!("{}", k),
			None => format!("Unknown"),
		}
	}
	/// get cause
	pub fn cause(&self) -> Option<&dyn Fail> {
		self.inner.cause()
	}
	/// get backtrace
	pub fn backtrace(&self) -> Option<&Backtrace> {
		self.inner.backtrace()
	}
}

impl From<proj::Error> for Error {
	fn from(error: proj::Error) -> Error {
		Error {
			inner: Context::new(ErrorKind::Proj(error.kind())),
		}
	}
}

impl From<vera::Error> for Error {
	fn from(error: vera::Error) -> Error {
		Error {
			inner: Context::new(ErrorKind::Vera(error.kind())),
		}
	}
}

impl From<ErrorKind> for Error {
	fn from(kind: ErrorKind) -> Error {
		Error {
			inner: Context::new(kind),
		}
	}
}

impl From<Context<ErrorKind>> for Error {
	fn from(inner: Context<ErrorKind>) -> Error {
		Error { inner: inner }
	}
}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Error {
		Error {
			inner: Context::new(ErrorKind::IO(format!("{}", error))),
		}
	}
}

impl From<ParseIntError> for Error {
	fn from(error: ParseIntError) -> Error {
		Error {
			inner: Context::new(ErrorKind::ParseIntError(error)),
		}
	}
}
