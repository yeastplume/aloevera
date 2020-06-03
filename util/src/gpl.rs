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

//! Implements basic support for reading Gimp .gpl files.

/// Parses a Gimp gpl file provided as a vector of bytes into a vector of (r,g,b) tuples.
pub fn parse_gpl_from_bytes(gpl_data: Vec<u8>) -> Result<Vec<(u8, u8, u8)>, &'static str> {
	let gpl_string = match String::from_utf8(gpl_data) {
		Ok(s) => s,
		_ => {
			return Err("Invalid gpl file content");
		}
	};

	fn is_comment(s: &str) -> bool {
		s.chars().skip_while(|c| c.is_whitespace()).next() == Some('#')
	}

	fn validate_line_1(s: &str) -> Result<(), &'static str> {
		if s != "GIMP Palette" {
			return Err("Invalid gpl file line 1");
		}
		return Ok(());
	}

	fn validate_line_2(s: &str) -> Result<(), &'static str> {
		if !s.starts_with("Name:") {
			return Err("Invalid gpl file line 2");
		}
		return Ok(());
	}

	fn validate_line_3(s: &str) -> Result<(), &'static str> {
		if !s.starts_with("Columns:") {
			return Err("Invalid gpl file line 3");
		}
		return Ok(());
	}

	fn parse_rgb_value(s: &str) -> Result<u8, &'static str> {
		match s.parse::<u8>() {
			Ok(n) => Ok(n),
			_ => Err("Failed to parse rgb value"),
		}
	}

	let mut colors = vec![];
	let mut line_num = 0;

	for line in gpl_string.lines() {
		line_num += 1;
		if is_comment(&line) || line.trim().len() == 0 {
			continue;
		}
		match line_num {
			1 => {
				validate_line_1(line)?;
			}
			2 => {
				validate_line_2(line)?;
			}
			3 => {
				validate_line_3(line)?;
			}
			_ => {
				let mut split = line.split_whitespace();
				match (split.next(), split.next(), split.next()) {
					(Some(r_str), Some(g_str), Some(b_str)) => {
						let r = parse_rgb_value(r_str)?;
						let g = parse_rgb_value(g_str)?;
						let b = parse_rgb_value(b_str)?;
						colors.push((r, g, b));
					}
					_ => {
						return Err("Invalid gpl file");
					}
				}
			}
		}
	}

	Ok(colors)
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_parse_gpl_from_bytes() {
		let test_gpl = include_bytes!("data/palette-gimp.gpl");

		match parse_gpl_from_bytes(test_gpl.to_vec()) {
			Ok(gpl) => {
				// Note: these values aren't yet scaled down to 4-bit values
				assert_eq!(gpl[0], (0, 0, 0));
				assert_eq!(gpl[2], (255, 255, 255));
				assert_eq!(gpl[4], (136, 102, 17));
				assert_eq!(gpl[10], (153, 153, 153));
				assert_eq!(gpl.len(), 16);
				println!("Read colors from palette:{:?}", gpl);
				println!("parse_gpl_from_bytes() succeeded on valid gpl file");
			}
			_ => {
				println!("parse_gpl_from_bytes() failed");
				assert_eq!(false, true)
			}
		}

		let invalid_gpl = include_bytes!("data/create.sh");

		match parse_gpl_from_bytes(invalid_gpl.to_vec()) {
			Err(_) => {
				println!("parse_gpl_from_bytes() failed as expected");
			}
			_ => {
				println!("parse_gpl_from_bytes() unexpectedly succeeded");
				assert_eq!(!false, true);
			}
		}
	}
}
