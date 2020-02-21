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
use fscommon::{BufStream, StreamSlice};
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

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

/// Write bytes to a file in the image
pub fn write_file_to_image(
	image_file: &str,
	file_name: &str,
	bytes: &[u8],
) -> Result<(), std::io::Error> {
	let md = fs::metadata(image_file)?;
	let img_file = OpenOptions::new().read(true).write(true).open(image_file)?;
	// TODO: The image files we're creating have their partition
	// aligned to 8MB, look into how to deal with this better
	//let first_lba = 8*1024*1024;
	let first_lba = 8 * 1024 * 1024;
	let last_lba = md.len() - first_lba;
	let partition = StreamSlice::new(img_file, first_lba, last_lba + 1)?;
	let buf_wrt = BufStream::new(partition);

	let options = fatfs::FsOptions::new().update_accessed_date(true);
	let fs = fatfs::FileSystem::new(buf_wrt, options)?;

	// have to create all directories manually
	// TODO: Fix unwrap messes
	let path = Path::new(file_name).to_path_buf();
	let name = path.file_name().unwrap().to_str();
	let mut path = Path::new(file_name).to_path_buf();
	path.pop();

	let mut path_str = String::from("");
	for d in path.iter() {
		let elem = d.to_str().unwrap();
		if elem == "." {
			continue;
		};
		path_str += &format!("{}", d.to_str().unwrap());
		fs.root_dir().create_dir(&path_str)?;
		path_str += "/";
	}

	path_str += name.unwrap();
	let mut file = fs.root_dir().create_file(&path_str)?;
	file.write_all(bytes)?;
	Ok(())
}
