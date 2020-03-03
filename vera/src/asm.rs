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

	/// Size, in bytes, of assembled asset
	fn size_in_bytes(&self, conflated_size: bool) -> Result<usize, Error>;
}

/// Enum for variations ToAsm implementors can assemble to
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AsmFormat {
	/// To Basic
	Basic,
	/// ca65 assembly
	Ca65,
	/// cc65-friendly header file
	Cc65,
	/// Raw Binary file
	Bin,
}

impl fmt::Display for AsmFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let out = match self {
			AsmFormat::Basic => "basic",
			AsmFormat::Ca65 => "ca65",
			AsmFormat::Cc65 => "cc65",
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
			"cc65" => Ok(AsmFormat::Cc65),
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
				AsmFormat::Cc65 => format!("{}\n", l),
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

/// Mostly for tilemaps, information on
/// how to conflate tilemap data to fit
/// actual VERA tilemap dimensions
/// starting address relative to start
/// of tilemap
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConflateInfo {
	/// start offset from start of vera tilemap
	start_offset: usize,
	/// number of tiles to draw per row
	stride: usize,
	/// number of tiles to skip per row
	skip: usize,
	/// Total vera tilemap length, in bytes
	tilemap_length: usize,
	/// Differing Meta
	conflated_meta: Vec<String>,
}

/// Holds raw assembled data, pre-formatting
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssembledPrimitive {
	/// Metadata about the struct, to be output
	/// as comments, a meta file or even code
	/// to load
	meta: Vec<String>,
	/// Assembled raw binary data
	data: Vec<u8>,
	/// Id, needed by some types of output
	id: String,
	/// Conflation data for tilemaps
	conflate_info: Option<ConflateInfo>,
}

impl AssembledPrimitive {
	/// New blank assembly
	pub fn new(id: &str) -> Self {
		Self {
			id: id.to_owned(),
			meta: vec![],
			data: vec![],
			conflate_info: None,
		}
	}

	/// set Tilemap values
	pub fn set_tilemap_conflate_info(
		&mut self,
		start_offset: u32,
		stride: u32,
		skip: u32,
		tilemap_length: u32,
		conflated_meta: Vec<String>,
	) {
		self.conflate_info = Some(ConflateInfo {
			start_offset: start_offset as usize,
			stride: stride as usize,
			skip: skip as usize,
			tilemap_length: tilemap_length as usize,
			conflated_meta,
		});
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

	/// retrieve bin data formatted as bin data
	pub fn data_as_bin(
		&self,
		address_bytes: Option<[u8; 2]>,
		conflate: bool,
	) -> Result<Vec<u8>, Error> {
		let mut data = self.data.clone();
		if self.conflate_info.is_some() && conflate {
			data = self.conflate_data()?;
		}
		//TODO: Some other method that doesn't involve cloning data
		let mut address_bytes = match address_bytes {
			Some(b) => b.to_vec(),
			None => [0, 0].to_vec(),
		};
		address_bytes.append(&mut data);
		Ok(address_bytes)
	}

	/// retrieve the raw bin data
	pub fn data_raw(&self) -> &Vec<u8> {
		&self.data
	}

	/// Conflate raw data
	fn conflate_data(&self) -> Result<Vec<u8>, Error> {
		let c_data = match self.conflate_info.clone() {
			Some(c) => c,
			None => {
				return Err(ErrorKind::InvalidAsmFormat("Missing Conflate Data".into()).into());
			}
		};
		// Add zeroes up to start index
		let mut ret_data = vec![0u8; c_data.start_offset];
		for i in (0..self.data.len()).step_by(c_data.stride) {
			let mut slice_vec = vec![0; c_data.stride];
			slice_vec.copy_from_slice(&self.data[i..i + c_data.stride]);
			ret_data.append(&mut slice_vec);
			for _ in 0..c_data.skip {
				ret_data.push(0);
			}
		}
		for _ in 0..(c_data.tilemap_length - ret_data.len()) {
			ret_data.push(0);
		}
		if ret_data.len() != c_data.tilemap_length {
			return Err(
				ErrorKind::InvalidAsmFormat("Conflated tilemap length is wrong".into()).into(),
			);
		}
		Ok(ret_data)
	}

	/// Output Meta, formatted for assembly target
	pub fn assemble_meta(
		&self,
		out_format: AsmFormat,
		conflate: bool,
	) -> Result<AssembledString, Error> {
		let mut retval = AssembledString::new(&out_format);
		let mut meta = &self.meta;
		let conf_meta;
		if self.conflate_info.is_some() && conflate {
			conf_meta = self.conflate_info.as_ref().unwrap().conflated_meta.clone();
			meta = &conf_meta;
		}
		if out_format == AsmFormat::Cc65 {
			retval.add(format!("/**"));
		}
		for m in meta.iter() {
			retval.add(match out_format {
				AsmFormat::Ca65 => format!(";{}", m),
				AsmFormat::Basic => format!("REM {}", m.to_uppercase()),
				AsmFormat::Cc65 => format!(" * {}", m),
				AsmFormat::Bin => format!(";{}", m),
			});
		}
		if out_format == AsmFormat::Cc65 {
			retval.add(format!(" */"));
		}
		Ok(retval)
	}

	/// Output data, formatted for assembly target
	pub fn assemble_data(
		&self,
		out_format: AsmFormat,
		conflate: bool,
	) -> Result<AssembledString, Error> {
		let mut data = &self.data;
		let conf_data;
		if self.conflate_info.is_some() && conflate {
			conf_data = self.conflate_data()?;
			data = &conf_data;
		}
		let mut retval = AssembledString::new(&out_format);
		if out_format == AsmFormat::Cc65 {
			retval.add(format!("#ifndef {}_H", self.id.to_uppercase()));
			retval.add(format!("#define {}_H", self.id.to_uppercase()));
			retval.add(format!(
				"static const unsigned char {}[] = {{",
				self.id.to_uppercase()
			));
		}
		for i in (0..data.len()).step_by(8) {
			let mut curval = String::from("");
			curval += &match out_format {
				AsmFormat::Ca65 => format!(".byte "),
				AsmFormat::Basic => format!("DATA "),
				AsmFormat::Cc65 => format!("    "),
				AsmFormat::Bin => {
					return Err(ErrorKind::InvalidAsmFormat(
						"Attempt to format binary data as string".into(),
					)
					.into());
				}
			};
			for j in 0..8 {
				let index = i + j;
				if index == data.len() {
					break;
				}
				curval += &match out_format {
					AsmFormat::Ca65 => format!("${:02X}", data[index]),
					AsmFormat::Basic => format!("{}", data[index]),
					AsmFormat::Cc65 => format!("0x{:02x}", data[index]),
					AsmFormat::Bin => {
						return Err(ErrorKind::InvalidAsmFormat(
							"Attempt to format binary data as string".into(),
						)
						.into());
					}
				};
				if (j < 7 || out_format == AsmFormat::Cc65) && index < data.len() - 1 {
					curval += ",";
				}
			}
			retval.add(curval);
		}
		if out_format == AsmFormat::Cc65 {
			retval.add(format!("}};"));
			retval.add(format!("#endif"));
		}
		Ok(retval)
	}
}

#[test]
fn test_assemble() -> Result<(), Error> {
	let mut prim = AssembledPrimitive::new("my_prim");
	prim.add_meta("here is some metadata 1".to_owned());
	prim.add_meta("here is some metadata 2".to_owned());
	prim.add_meta("here is some metadata 4".to_owned());
	prim.add_data(&[16u8; 34]);

	let mut line_count = 1;
	let meta_strs = prim.assemble_meta(AsmFormat::Basic, false)?;
	let num_lines = meta_strs.line_count();
	let meta_str = meta_strs.to_string(Some(line_count))?;
	line_count += num_lines;

	let data_strs = prim.assemble_data(AsmFormat::Basic, false)?;
	let data_str = data_strs.to_string(Some(line_count))?;

	println!("Meta Basic");
	println!("{}", meta_str);

	println!("Data Basic");
	println!("{}", data_str);
	assert!(data_str.ends_with("8 DATA 16,16\n"));

	let meta_strs = prim.assemble_meta(AsmFormat::Ca65, false)?;
	let meta_str = meta_strs.to_string(None)?;

	let data_strs = prim.assemble_data(AsmFormat::Ca65, false)?;
	let data_str = data_strs.to_string(None)?;

	println!("Meta Ca65");
	println!("{}", meta_str);

	println!("Data Ca65");
	println!("{}", data_str);

	assert!(data_str.starts_with(".byte $10,"));
	assert!(data_str.ends_with(".byte $10,$10\n"));

	Ok(())
}
