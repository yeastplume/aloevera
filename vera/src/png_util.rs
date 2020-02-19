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

use crate::VeraImage;
use crate::{Error, ErrorKind};

/// Load png frames tiled
pub fn png_to_frames(
	id: &str,
	f_width: u32,
	f_height: u32,
	png_data: Vec<u8>,
	result: &mut Vec<VeraImage>,
) -> Result<(u32, u32), Error> {
	let decoder = png::Decoder::new(&*png_data);
	let (info, mut reader) = decoder.read_info()?;
	if info.width % f_width != 0 || info.height % f_height != 0 {
		return Err(ErrorKind::PNGIncorrectDimensions(
			info.width,
			info.height,
			f_width as u32,
			f_height as u32,
		)
		.into());
	}
	debug!("Decoded PNG Info: {:?}", info);
	let mut buf = vec![0; info.buffer_size()];
	reader.next_frame(&mut buf)?;
	trace!("buf data is: {:?}", buf);

	let frames_per_row = info.width / f_width;
	let frames_per_col = info.height / f_height;
	let frame_count = frames_per_col * frames_per_row;

	if info.bit_depth == png::BitDepth::Sixteen {
		return Err(ErrorKind::PNGInvalid(format!("PNG must be 8 bit color depth or less")).into());
	}

	let step = info.color_type.samples();

	// read data png again, saving palette indices in the appropriate places
	info!("Parsing {} frames", frame_count);
	for i in 0..frame_count {
		result.push(VeraImage::new(&format!("{}_{}", id, i), f_width, f_height));
	}
	for i in (0..buf.len()).step_by(step) {
		let pixel_loc = i / step;
		let row = pixel_loc % buf.len() / info.width as usize;
		let col = pixel_loc - row * info.width as usize;
		let frame_y = row / f_height as usize;
		let frame_x = col / f_width as usize;
		// Just load raw RGB values for now, and reconcile with a palette/depth
		// later
		result[frame_y * frames_per_row as usize + frame_x].push_pixel(
			buf[i],
			buf[i + 1],
			buf[i + 2],
			None,
		);
	}

	Ok((frames_per_row, frames_per_col))
}
