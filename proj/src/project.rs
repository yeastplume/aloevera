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
//! Top Level Project file definition

use crate::Binable;
use crate::Error;
use std::collections::BTreeMap;
use vera::{VeraBitmap, VeraImageSet, VeraPalette, VeraSprite, VeraTileMap};

/// Top level project file definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AloeVeraProject<'a> {
	/// Internal Id for the project
	pub id: String,
	/// Palettes, Ids mapped to pallete info
	pub palettes: BTreeMap<String, VeraPalette>,
	/// Palettes, Ids mapped to imageset info
	pub imagesets: BTreeMap<String, VeraImageSet>,
	/// Tilemaps, which much be matched to imagesets
	pub tilemaps: BTreeMap<String, VeraTileMap>,
	/// Sprites, which are bounds-checking wrappers around Imagesets
	pub sprites: BTreeMap<String, VeraSprite<'a>>,
	/// Bitmaps, which are bounds-checking wrappers around Imagesets
	pub bitmaps: BTreeMap<String, VeraBitmap<'a>>,
}

impl<'a> Binable for AloeVeraProject<'a> {
	fn to_bin(&self) -> Result<Vec<u8>, Error> {
		let encoded = bincode::serialize(&self)?;
		Ok(encoded)
	}

	fn from_bin(encoded: &Vec<u8>) -> Result<Box<Self>, Error> {
		let decoded = bincode::deserialize(&encoded[..])?;
		Ok(Box::new(decoded))
	}
}

impl<'a> AloeVeraProject<'a> {
	/// create a new project
	pub fn new(id: &str) -> Self {
		Self {
			id: id.into(),
			palettes: BTreeMap::new(),
			imagesets: BTreeMap::new(),
			tilemaps: BTreeMap::new(),
			sprites: BTreeMap::new(),
			bitmaps: BTreeMap::new(),
		}
	}
}
