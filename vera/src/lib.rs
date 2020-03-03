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

//! Types and transformation operations on VERA data structures

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

#[macro_use]
extern crate log;
extern crate aloevera_util as util;

#[macro_use]
extern crate serde_derive;

mod asm;
mod bitmap;
mod error;
mod imageset;
mod palette;
mod png_util;
mod sprite;
mod tilemap;

pub use asm::{AsmFormat, Assemblable, AssembledPrimitive};
pub use bitmap::VeraBitmap;
pub use error::{Error, ErrorKind};
pub use imageset::{VeraImage, VeraImageSet, VeraImageSetLoadConfig, VeraPixelDepth};
pub use palette::{VeraPalette, VeraPaletteEntry, VeraPaletteLoadConfig};
pub use png_util::png_to_frames;
pub use sprite::VeraSprite;
pub use tilemap::{VeraTileMap, VeraTileMapDim, VeraTileMapEntry, VeraTileMapMode};
