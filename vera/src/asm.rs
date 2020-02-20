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

//! Holds intermediate assembly of Aloevera primitives,
//! which can then be given their final transformations
//! to target output

use crate::{Error, ErrorKind};
use std::fmt;
use std::str::FromStr;

/// Trait for object that outputs its data to an assembled format
pub trait Assemblable {
	/// Assemble to raw byte data + meta data statements
	fn assemble(&self) -> Result<AssembledPrimitive, Error>;

	/// Return ID of assembled asset
	fn id(&self) -> &str;
}

/// Enum for variations ToAsm implementors can assemble to
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AsmFormat {
	/// To Basic
	Basic,
	/// ca65 assembly
	Ca65,
	/// Raw Binary file
	Bin,
}

impl fmt::Display for AsmFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let out = match self {
			AsmFormat::Basic => "basic",
			AsmFormat::Ca65 => "ca65",
			AsmFormat::Bin => "bin",
		};
		write!(f, "{}", out)
	}
}

/// From String
impl FromStr for AsmFormat {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"ca65" => Ok(AsmFormat::Ca65),
			"basic" => Ok(AsmFormat::Basic),
			"bin" => Ok(AsmFormat::Bin),
			other => Err(ErrorKind::UnknownAsmFormat(other.to_owned()).into()),
		}
	}
}

/// Holds and can format 'assembled' strings
#[derive(Clone, Debug)]
pub struct AssembledString {
	/// Target format
	target_format: AsmFormat,
	/// to load
	assembled_data: Vec<String>,
}

impl AssembledString {
	/// New blank assembly
	pub fn new(target_format: &AsmFormat) -> Self {
		Self {
			target_format: target_format.clone(),
			assembled_data: vec![],
		}
	}

	/// Add meta information
	pub fn add(&mut self, new_meta: String) {
		self.assembled_data.push(new_meta);
	}

	/// Output final string
	pub fn to_string(self, line_start: Option<usize>) -> Result<String, Error> {
		let line_start = match line_start {
			Some(v) => v,
			None => 1,
		};
		let mut retval = String::from("");
		for (i, l) in self.assembled_data.iter().enumerate() {
			retval += &match self.target_format {
				AsmFormat::Ca65 => format!("{}\n", l),
				AsmFormat::Basic => format!("{} {}\n", line_start + i, l),
				// only metadata should get to this stage
				AsmFormat::Bin => format!("{}\n", l),
			}
		}
		Ok(retval)
	}

	/// number of lines
	pub fn line_count(&self) -> usize {
		self.assembled_data.len()
	}
}

/// Holds raw assembled data, pre-formatting
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssembledPrimitive {
	/// Metadata about the struct, to be output
	/// as comments, a meta file or even code
	/// to load
	meta: Vec<String>,
	/// Assembled raw binary data
	pub data: Vec<u8>,
}

impl AssembledPrimitive {
	/// New blank assembly
	pub fn new() -> Self {
		Self {
			meta: vec![],
			data: vec![],
		}
	}

	/// Add another prim to this one, consuming second
	pub fn add_prim(&mut self, mut other: AssembledPrimitive) {
		self.meta.append(&mut other.meta);
		self.data.append(&mut other.data);
	}

	/// Add meta information
	pub fn add_meta(&mut self, new_meta: String) {
		self.meta.push(new_meta);
	}

	/// Add assembled byte data
	pub fn add_data(&mut self, new_data: &[u8]) {
		for b in new_data {
			self.data.push(*b);
		}
	}

	/// Output Meta, formatted for assembly target
	pub fn assemble_meta(&self, out_format: AsmFormat) -> Result<AssembledString, Error> {
		let mut retval = AssembledString::new(&out_format);
		for m in self.meta.iter() {
			retval.add(match out_format {
				AsmFormat::Ca65 => format!(";{}", m),
				AsmFormat::Basic => format!("REM {}", m.to_uppercase()),
				AsmFormat::Bin => format!(";{}", m),
			});
		}
		Ok(retval)
	}

	/// Output data, formatted for assembly target
	pub fn assemble_data(&self, out_format: AsmFormat) -> Result<AssembledString, Error> {
		let mut retval = AssembledString::new(&out_format);
		for i in (0..self.data.len()).step_by(8) {
			let mut curval = String::from("");
			curval += &match out_format {
				AsmFormat::Ca65 => format!(".byte "),
				AsmFormat::Basic => format!("DATA "),
				AsmFormat::Bin => {
					return Err(ErrorKind::InvalidAsmFormat(
						"Attempt to format binary data as string".into(),
					)
					.into());
				}
			};
			for j in 0..8 {
				let index = i + j;
				if index == self.data.len() {
					break;
				}
				curval += &match out_format {
					AsmFormat::Ca65 => format!("${:02X}", self.data[index]),
					AsmFormat::Basic => format!("{}", self.data[index]),
					AsmFormat::Bin => {
						return Err(ErrorKind::InvalidAsmFormat(
							"Attempt to format binary data as string".into(),
						)
						.into());
					}
				};
				if j < 7 && index < self.data.len() - 1 {
					curval += ",";
				}
			}
			retval.add(curval);
		}
		Ok(retval)
	}
}

#[test]
fn test_assemble() -> Result<(), Error> {
	let mut prim = AssembledPrimitive::new();
	prim.add_meta("here is some metadata 1".to_owned());
	prim.add_meta("here is some metadata 2".to_owned());
	prim.add_meta("here is some metadata 4".to_owned());
	prim.add_data(&[16u8; 34]);

	let mut line_count = 1;
	let meta_strs = prim.assemble_meta(AsmFormat::Basic)?;
	let num_lines = meta_strs.line_count();
	let meta_str = meta_strs.to_string(Some(line_count))?;
	line_count += num_lines;

	let data_strs = prim.assemble_data(AsmFormat::Basic)?;
	let data_str = data_strs.to_string(Some(line_count))?;

	println!("Meta Basic");
	println!("{}", meta_str);

	println!("Data Basic");
	println!("{}", data_str);
	assert!(data_str.ends_with("8 DATA 16,16\n"));

	let meta_strs = prim.assemble_meta(AsmFormat::Ca65)?;
	let meta_str = meta_strs.to_string(None)?;

	let data_strs = prim.assemble_data(AsmFormat::Ca65)?;
	let data_str = data_strs.to_string(None)?;

	println!("Meta Ca65");
	println!("{}", meta_str);

	println!("Data Ca65");
	println!("{}", data_str);

	assert!(data_str.starts_with(".byte $10,"));
	assert!(data_str.ends_with(".byte $10,$10\n"));

	Ok(())
}
