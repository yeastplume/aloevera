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

//! Utilities to write to FAT Filesystem images, for eventual
//! transfer to SD Cards

use fatfs;
use flate2::write::GzDecoder;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

/// Store our template fat32 image nice and compressed
/// in the binary, (it's mostly zeroes so it compresses well)
const FAT32_TEMPLATE_IMAGE: &'static [u8] = include_bytes!("data/fat32_template.img.gz");

/// Create a new fat32 image in the given file, essentially decompressing
/// from an embedded sample image
pub fn create_fat_image(filename: &str) -> Result<(), std::io::Error> {
	let stream = BufWriter::new(File::create(filename)?);
	let mut decoder = GzDecoder::new(stream);
	decoder.write_all(FAT32_TEMPLATE_IMAGE)?;
	decoder.try_finish()?;
	Ok(())
}
